use sqlx::{PgPool, QueryBuilder};
use uuid::Uuid;

use crate::dto::page::{PageQuery, PageResponse};
use crate::dto::video_detail::VideoDetail;
use crate::models::video::Video;
use crate::services::video_image_service::VideoImageService;

const DEFAULT_SIZE: i64 = 10;
const MAX_SIZE: i64 = 100;

pub async fn find_videos_paginated(
    pool: &PgPool,
    q: &PageQuery,
) -> Result<PageResponse<Video>, sqlx::Error> {

    let page = q.page.unwrap_or(0).max(0);

    let mut size = q.size.unwrap_or(DEFAULT_SIZE);
    if size <= 0 {
        size = DEFAULT_SIZE;
    }
    if size > MAX_SIZE {
        size = MAX_SIZE;
    }

    let offset = page * size;

    // =========================
    // COUNT
    // =========================
    let mut count_builder = QueryBuilder::new(
        "SELECT COUNT(*) FROM videos WHERE 1=1"
    );

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
        .await?;

    // =========================
    // DATA
    // =========================
    let mut data_builder = QueryBuilder::new(
        "SELECT id, title, extension, size_mb, published_at, duration_seconds, resolution, video_height, image_url \
         FROM videos WHERE 1=1"
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
        .await?;

    Ok(PageResponse {
        items,
        total,
        page,
        size,
    })
}

pub async fn find_video_by_uuid(
    pool: &PgPool,
    uuid: Uuid,
    image_service: &VideoImageService,
) -> Result<Option<VideoDetail>, Box<dyn std::error::Error + Send + Sync>> {
    let video = sqlx::query_as::<_, Video>(
        r#"
        SELECT id, title, extension, size_mb, published_at,
               duration_seconds, resolution, video_height, image_url
        FROM videos
        WHERE id = $1
        "#
    )
    .bind(uuid)
    .fetch_optional(pool)
    .await?;

    let Some(video) = video else {
        return Ok(None);
    };

    let uuid_str = uuid.to_string();

    tracing::info!(uuid = %uuid_str, "fetching main image from bucket");

    let main_image_url = match image_service
        .get_main_image_presigned_if_exists(&uuid_str, 3600)
        .await
    {
        Ok(Some(url)) => {
            tracing::info!(
                uuid = %uuid_str,
                found = true,
                "main image lookup complete"
            );
            Some(url)
        }
        Ok(None) => {
            tracing::warn!(
                uuid = %uuid_str,
                found = false,
                "main image not found in bucket"
            );
            None
        }
        Err(e) => {
            tracing::warn!(
                uuid = %uuid_str,
                error = ?e,
                "error checking main image in bucket"
            );
            None
        }
    };

    tracing::info!(uuid = %uuid_str, "fetching scene images from bucket");

    let scene_images = match image_service
        .get_scene_images_presigned(&uuid_str, 3600)
        .await
    {
        Ok(images) => images,
        Err(e) => {
            tracing::warn!(
                uuid = %uuid_str,
                error = ?e,
                "error fetching scene images from bucket"
            );
            Vec::new()
        }
    };

    tracing::info!(
        uuid = %uuid_str,
        count = scene_images.len(),
        "scene images lookup complete"
    );

    Ok(Some(VideoDetail {
        id: video.id,
        title: video.title,
        extension: video.extension.unwrap_or_default(),
        size_mb: video.size_mb
            .map(|d| d.to_string().parse::<f64>().unwrap_or(0.0))
            .unwrap_or(0.0),
        published_at: video.published_at,
        duration_seconds: video.duration_seconds,
        resolution: video.resolution,
        video_height: video.video_height.and_then(|h| h.parse::<i32>().ok()),
        image_url: video.image_url,
        main_image_url,
        scene_images,
    }))
}