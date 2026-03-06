use axum::{
    extract::{Query, State},
    http::StatusCode,
    Json,
};
use sqlx::QueryBuilder;

use crate::dto::page::{PageQuery, PageResponse};
use crate::models::video::Video;
use crate::routes::AppState; // ajusta el path si AppState está en otro módulo

const DEFAULT_SIZE: i64 = 10;
const MAX_SIZE: i64 = 100;

pub async fn get_videos(
    State(state): State<AppState>,
    Query(q): Query<PageQuery>,
) -> Result<Json<PageResponse<Video>>, (StatusCode, String)> {
    let pool = &state.pool;

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
    let mut count_builder = QueryBuilder::new("SELECT COUNT(*) FROM videos WHERE 1=1");

    if let Some(title) = &q.title {
        let t = title.trim();
        if !t.is_empty() {
            count_builder
                .push(" AND title ILIKE ")
                .push_bind(format!("%{}%", t));
        }
    }

    let total: i64 = count_builder
        .build_query_scalar()
        .fetch_one(pool)
        .await
        .map_err(|e| {
            tracing::error!(error=?e, "DB error (count)");
            (StatusCode::INTERNAL_SERVER_ERROR, "db error (count)".to_string())
        })?;

    // =========================
    // DATA (ITEMS)
    // =========================
    let mut data_builder = QueryBuilder::new(
        "SELECT id, title, extension, size_mb, published_at, duration_seconds, resolution, video_height, image_url \
         FROM videos WHERE 1=1",
    );

    if let Some(title) = &q.title {
        let t = title.trim();
        if !t.is_empty() {
            data_builder
                .push(" AND title ILIKE ")
                .push_bind(format!("%{}%", t));
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
        .fetch_all(pool)
        .await
        .map_err(|e| {
            tracing::error!(error=?e, "DB error (items)");
            (StatusCode::INTERNAL_SERVER_ERROR, "db error (items)".to_string())
        })?;

    Ok(Json(PageResponse { items, total, page, size }))
}