use axum::Router;
use sqlx::PgPool;

use http::{header, Method};
use tower_http::cors::{Any, CorsLayer};

mod video;

pub fn create_routes(pool: PgPool) -> Router {
    // ✅ DEV: permite cualquier Origin (incluye cualquier puerto en localhost)
    // ⚠️ No usar en PROD si te importa seguridad.
    // ⚠️ No sirve si usas cookies/credentials.
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods([Method::GET, Method::POST, Method::PUT, Method::DELETE, Method::OPTIONS])
        .allow_headers([header::CONTENT_TYPE, header::AUTHORIZATION]);

    Router::new()
        .nest(
            "/api/v1",
            Router::new()
                .nest("/videos", video::routes()),
        )
        .with_state(pool)
        .layer(cors)
}