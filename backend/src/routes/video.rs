use axum::{routing::get, Router};
use crate::handlers::video::get_videos;
use crate::routes::AppState;

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/", get(get_videos))
}