use axum::{extract::State, Json};
use sqlx::PgPool;
use crate::models::video::Video;

pub async fn get_videos(State(pool): State<PgPool>) -> Json<Vec<Video>> {
    let rows = sqlx::query_as::<_, Video>("SELECT * FROM videos ORDER BY title")
        .fetch_all(&pool)
        .await
        .unwrap_or_else(|_| vec![]);
    Json(rows)
}