use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    Json,
};
use sqlx::QueryBuilder;
use uuid::Uuid;

use crate::dto::page::{PageQuery, PageResponse};
use crate::dto::video_detail::VideoDetail;
use crate::models::video::Video;
use crate::routes::AppState;
use crate::services::video_image_service::VideoImageService;
use crate::services::video_service;
const DEFAULT_SIZE: i64 = 10;
const MAX_SIZE: i64 = 100;

pub async fn get_videos(
    State(state): State<AppState>,
    Query(q): Query<PageQuery>,
) -> Result<Json<PageResponse<Video>>, (StatusCode, String)> {

    tracing::info!(
        "get_videos → page={:?}, size={:?}, title={:?}",
        q.page, q.size, q.title
    );

    let result = video_service::find_videos_paginated(&state.pool, &q)
        .await
        .map_err(|e| {
            tracing::error!(error=?e, "DB error (videos)");
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                "db error (videos)".to_string(),
            )
        })?;

    Ok(Json(result))
}

pub async fn get_video_by_uuid(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<Json<VideoDetail>, (StatusCode, String)> {

    let image_service = VideoImageService::new(state.s3.clone(), state.minio_bucket.clone());

    let video = video_service::find_video_by_uuid(&state.pool, id, &image_service)
        .await
        .map_err(|e| {
            tracing::error!(error=?e, id=%id, "error fetching video by id");
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                "error fetching video by id".to_string(),
            )
        })?;

    match video {
        Some(video) => Ok(Json(video)),
        None => Err((StatusCode::NOT_FOUND, format!("video {} not found", id))),
    }
}