use axum::{
    http::{header, Method},
    Router,
};
use sqlx::PgPool;
use tower_http::cors::{Any, CorsLayer};

use aws_sdk_s3::Client as S3Client;
use crate::config::Settings;

mod video;

#[derive(Clone)]
pub struct AppState {
    pub pool: PgPool,
    pub settings: Settings,
    pub s3: S3Client,
}

pub fn create_routes(state: AppState) -> Router {
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods([Method::GET, Method::POST, Method::PUT, Method::DELETE, Method::OPTIONS])
        .allow_headers([header::CONTENT_TYPE, header::AUTHORIZATION]);

    Router::new()
        .nest("/api/v1", Router::new().nest("/videos", video::routes()))
        .with_state(state)
        .layer(cors)
}