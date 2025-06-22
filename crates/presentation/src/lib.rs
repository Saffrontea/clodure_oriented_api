use application::{UserRepository, UserService};
use domain::{CreateUserRequest, UpdateUserRequest, UserError};
use std::convert::Infallible;
use std::sync::Arc;
use uuid::Uuid;
use warp::{Filter, Rejection, Reply};

// 関数合成スタイルでフィルターを構築するヘルパー関数
pub fn with_service<R: UserRepository + Send + Sync + 'static>(
    service: Arc<UserService<R>>,
) -> impl Filter<Extract = (Arc<UserService<R>>,), Error = Infallible> + Clone {
    warp::any().map(move || service.clone())
}

pub fn user_error_to_response(err: UserError) -> Rejection {
    use warp::http::StatusCode;
    use warp::reject::custom;

    let (_code, message) = match err {
        UserError::NotFound(_) => (StatusCode::NOT_FOUND, err.to_string()),
        UserError::ValidationError(_) => (StatusCode::BAD_REQUEST, err.to_string()),
        UserError::DatabaseError(_) => (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()),
    };

    #[derive(Debug)]
    struct ApiErrorMessage(String);
    impl warp::reject::Reject for ApiErrorMessage {}

    custom(ApiErrorMessage(message))
}

// エラーをレスポンスに変換する関数
async fn handle_rejection(err: Rejection) -> Result<impl Reply, Infallible> {
    use warp::http::StatusCode;

    #[derive(Debug)]
    struct ApiError {
        code: StatusCode,
        message: String,
    }

    impl warp::reject::Reject for ApiError {}

    let (code, message) = if let Some(api_err) = err.find::<ApiError>() {
        (api_err.code, api_err.message.clone())
    } else {
        (StatusCode::INTERNAL_SERVER_ERROR, "不明なエラーが発生しました".to_string())
    };

    let json = warp::reply::json(&serde_json::json!({ "error": message }));
    Ok(warp::reply::with_status(json, code))
}

pub fn create_user_api<R: UserRepository + Send + Sync + 'static>(
    service: Arc<UserService<R>>,
) -> impl Filter<Extract = impl Reply, Error = Infallible> + Clone {
    // 各エンドポイントを純粋な関数合成で実装
    let get_all_users = warp::path("users")
        .and(warp::get())
        .and(with_service(service.clone()))
        .and_then(|service: Arc<UserService<R>>| async move {
            match service.get_users().await {
                Ok(users) => Ok(warp::reply::json(&users)),
                Err(err) => Err(user_error_to_response(err))
            }
        });

    let get_user_by_id = warp::path!("users" / String)
        .and(warp::get())
        .and(with_service(service.clone()))
        .and_then(|id_str: String, service: Arc<UserService<R>>| async move {
            let id = match Uuid::parse_str(&id_str) {
                Ok(id) => id,
                Err(_) => return Err(user_error_to_response(UserError::ValidationError("無効なID形式".to_string())))
            };

            match service.get_user(id).await {
                Ok(user) => Ok(warp::reply::json(&user)),
                Err(err) => Err(user_error_to_response(err))
            }
        });

    let create_user = warp::path("users")
        .and(warp::post())
        .and(warp::body::json())
        .and(with_service(service.clone()))
        .and_then(|req: CreateUserRequest, service: Arc<UserService<R>>| async move {
            match service.create_user(req).await {
                Ok(user) => Ok(warp::reply::with_status(
                    warp::reply::json(&user),
                    warp::http::StatusCode::CREATED
                )),
                Err(err) => Err(user_error_to_response(err))
            }
        });

    let update_user = warp::path!("users" / String)
        .and(warp::put())
        .and(warp::body::json())
        .and(with_service(service.clone()))
        .and_then(|id_str: String, req: UpdateUserRequest, service: Arc<UserService<R>>| async move {
            let id = match Uuid::parse_str(&id_str) {
                Ok(id) => id,
                Err(_) => return Err(user_error_to_response(UserError::ValidationError("無効なID形式".to_string())))
            };

            match service.update_user(id, req).await {
                Ok(user) => Ok(warp::reply::json(&user)),
                Err(err) => Err(user_error_to_response(err))
            }
        });

    let delete_user = warp::path!("users" / String)
        .and(warp::delete())
        .and(with_service(service.clone()))
        .and_then(|id_str: String, service: Arc<UserService<R>>| async move {
            let id = match Uuid::parse_str(&id_str) {
                Ok(id) => id,
                Err(_) => return Err(user_error_to_response(UserError::ValidationError("無効なID形式".to_string())))
            };

            match service.delete_user(id).await {
                Ok(_) => Ok(warp::reply::with_status(
                    warp::reply::json(&serde_json::json!({"message": "ユーザーを削除しました"})),
                    warp::http::StatusCode::OK
                )),
                Err(err) => Err(user_error_to_response(err))
            }
        });

    // すべてのルートを合成
    let routes = get_user_by_id
        .or(get_all_users)
        .or(create_user)
        .or(update_user)
        .or(delete_user)
        .with(warp::cors().allow_any_origin().allow_methods(vec!["GET", "POST", "PUT", "DELETE"]));

    // エラーハンドリングを追加
    routes.recover(handle_rejection)
}
