use std::net::SocketAddr;

use axum_jwt::config;
use clap::Clap;
use tracing_subscriber::EnvFilter;
use log::{info};
#[tokio::main]
async fn main() {
    use config::db::DbPool;

    if dotenv::dotenv().is_err() {
        tracing::warn!("missing .env file in current working directory");
    };
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .pretty()
        .init();

    let pg_pool = sqlx::PgPool::retrieve().await;
    let config = config::env::ServerConfig::parse();
    let addr = SocketAddr::from((config.host, config.port));
    info!("Razor located: {}", addr);
    tracing::debug!("listening on {}", addr);
    let server =
        axum::Server::bind(&addr).serve(axum_jwt::app(pg_pool).into_make_service());

    if let Err(err) = server.await {
        tracing::error!("server error: {:?}", err);
    }
}
