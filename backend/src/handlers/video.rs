use axum::{extract::{Query, State}, Json};
use sqlx::PgPool;

use crate::models::video::Video;
use crate::dto::page::{PageQuery, PageResponse};

const DEFAULT_SIZE: i64 = 10;
const MAX_SIZE: i64 = 100;

pub async fn get_videos(
    State(pool): State<PgPool>,
    Query(q): Query<PageQuery>,
) -> Json<PageResponse<Video>> {
    let page = q.page.unwrap_or(0).max(0);
    let mut size = q.size.unwrap_or(DEFAULT_SIZE);
    if size <= 0 { size = DEFAULT_SIZE; }
    if size > MAX_SIZE { size = MAX_SIZE; }

    let offset = page * size;

    tracing::info!(
        "get_videos called → page={}, size={}, offset={}",
        page, size, offset
    );    

    // 1) total
    let total: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM videos")
        .fetch_one(&pool)
        .await
        .unwrap_or(0);

    // 2) items
    let items: Vec<Video> = sqlx::query_as::<_, Video>(
        "SELECT * FROM videos ORDER BY title LIMIT $1 OFFSET $2"
    )
    .bind(size)
    .bind(offset)
    .fetch_all(&pool)
    .await
    .unwrap_or_default();

    Json(PageResponse { items, total, page, size })
}