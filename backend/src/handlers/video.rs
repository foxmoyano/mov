use axum::{extract::{Query, State}, Json};
use sqlx::{PgPool, QueryBuilder};

use crate::models::video::Video;
use crate::dto::page::{PageQuery, PageResponse};

const DEFAULT_SIZE: i64 = 10;
const MAX_SIZE: i64 = 100;

pub async fn get_videos(
    State(pool): State<PgPool>,
    Query(q): Query<PageQuery>,
) -> Json<PageResponse<Video>> {

    // =========================
    // PAGINACIÓN
    // =========================
    let page = q.page.unwrap_or(0).max(0);

    let mut size = q.size.unwrap_or(DEFAULT_SIZE);
    if size <= 0 {
        size = DEFAULT_SIZE;
    }
    if size > MAX_SIZE {
        size = MAX_SIZE;
    }

    let offset = page * size;

    tracing::info!(
        "get_videos → page={}, size={}, offset={}, title={:?}",
        page, size, offset, q.title
    );

    // =========================
    // COUNT (TOTAL)
    // =========================
    let mut count_builder =
        QueryBuilder::new("SELECT COUNT(*) FROM videos WHERE 1=1");

    if let Some(title) = &q.title {
        if !title.trim().is_empty() {
            count_builder
                .push(" AND title ILIKE ")
                .push_bind(format!("%{}%", title));
        }
    }

    let total: i64 = count_builder
        .build_query_scalar()
        .fetch_one(&pool)
        .await
        .unwrap_or(0);

    // =========================
    // DATA (ITEMS)
    // =========================
    let mut data_builder = QueryBuilder::new(
        "SELECT id, title, extension, size_mb, published_at FROM videos WHERE 1=1"
    );

    if let Some(title) = &q.title {
        if !title.trim().is_empty() {
            data_builder
                .push(" AND title ILIKE ")
                .push_bind(format!("%{}%", title));
        }
    }

    data_builder
        .push(" ORDER BY title ")
        .push(" LIMIT ")
        .push_bind(size)
        .push(" OFFSET ")
        .push_bind(offset);

    let items: Vec<Video> = data_builder
        .build_query_as()
        .fetch_all(&pool)
        .await
        .unwrap_or_default();

    // =========================
    // RESPONSE
    // =========================
    Json(PageResponse {
        items,
        total,
        page,
        size,
    })
}