use axum::async_trait;
use clap::Clap;
use sqlx::{postgres::PgPoolOptions, PgPool};

use crate::config::{db::DbPool, env::PgConfig};

#[async_trait]
impl DbPool for PgPool {
    async fn retrieve() -> Self {
        let config = PgConfig::parse();
        let uri = format!(
            "postgres://postgres:123456@127.0.0.1:5432/postgres"
            // config.pg_user, config.pg_password, config.pg_host, config.pg_port, config.pg_database
        );

        PgPoolOptions::new()
            .connect(&uri)
            .await
            .expect("DB connection was failed")
    }
}
