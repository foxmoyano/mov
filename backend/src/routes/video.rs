use axum::{routing::get, Router};
use crate::handlers::video::{get_videos, get_video_by_uuid};

use crate::routes::AppState;

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/", get(get_videos))
        .route("/:id", get(get_video_by_uuid))
}