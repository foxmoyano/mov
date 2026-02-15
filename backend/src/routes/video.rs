use axum::{routing::get, Router};
use sqlx::PgPool;

use crate::handlers::video::get_videos;

pub fn routes() -> Router<PgPool> {
    Router::new()
        .route("/", get(get_videos))
}