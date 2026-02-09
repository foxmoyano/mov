use axum::{Router};
use sqlx::PgPool;

use crate::handlers::video::get_videos;

pub fn create_routes(pool: PgPool) -> Router {
    Router::new()
        .route("/videos", axum::routing::get(get_videos))
        .with_state(pool)
}

