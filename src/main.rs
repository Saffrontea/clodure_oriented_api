use warp::Filter;
use application::UserService;
use cores::shared;
use infrastructure::MySqlUserRepository;
use presentation::create_user_api;
use r2d2::Pool;
use r2d2_mysql::mysql::{Opts, OptsBuilder};
use r2d2_mysql::MySqlConnectionManager;
use tracing::info;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // ログ設定
    tracing_subscriber::fmt::init();
    info!("APIサーバーの起動準備中...");

    // データベース接続設定
    let database_url = std::env::var("DATABASE_URL").unwrap_or_else(|_| "mysql://user:password@db:3306/clodure_db".to_string());
    let opts = Opts::from_url(&database_url).expect("Invalid DATABASE_URL");
    let builder = OptsBuilder::from_opts(opts);
    let manager = MySqlConnectionManager::new(builder);
    let pool = Pool::new(manager)?;
    let pool = shared(pool);

    // レポジトリの初期化
    let repository = MySqlUserRepository::new(pool);
    repository.init_db()?;
    let repository = shared(repository);

    // サービスの初期化
    let service = UserService::new(repository);
    let service = shared(service);

    // APIルートの構築
    let api = create_user_api(service);

    // 最終的なルーターを合成
    let routes = api.with(warp::log("api"));

    // サーバーの起動
    let addr: std::net::SocketAddr = ([0,0,0,0], 3030).into();
    info!("サーバーを起動しました http://{}", addr);
    warp::serve(routes).run(addr).await;

    Ok(())
}