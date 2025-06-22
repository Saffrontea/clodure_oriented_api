use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: Uuid,
    pub name: String,
    pub email: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateUserRequest {
    pub name: String,
    pub email: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateUserRequest {
    pub name: Option<String>,
    pub email: Option<String>,
}

#[derive(Debug, thiserror::Error)]
pub enum UserError {
    #[error("ユーザーが見つかりません: {0}")]
    NotFound(Uuid),
    #[error("データベースエラー: {0}")]
    DatabaseError(String),
    #[error("検証エラー: {0}")]
    ValidationError(String),
}
