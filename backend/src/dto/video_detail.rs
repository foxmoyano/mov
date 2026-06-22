use chrono::NaiveDateTime;
use serde::Serialize;
use uuid::Uuid;

#[derive(Debug, Serialize)]
pub struct VideoDetail {
    pub id: Uuid,
    pub title: String,
    pub extension: String,
    pub size_mb: f64,
    pub published_at: Option<NaiveDateTime>,
    pub duration_seconds: Option<i32>,
    pub resolution: Option<String>,
    pub video_height: Option<i32>,
    pub image_url: Option<String>,

    // imágenes obtenidas desde MinIO
    pub main_image_url: Option<String>,
    pub scene_images: Vec<String>,
}