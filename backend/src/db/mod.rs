use sqlx::postgres::PgPoolOptions;
use crate::config;

pub async fn init() -> Result<sqlx::PgPool, sqlx::Error> {
    let database_url = config::database_url();
    PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
}

