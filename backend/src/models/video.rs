use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;
use rust_decimal::Decimal;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Video {
    pub id: Uuid,
    pub title: String,
    pub extension: Option<String>,
    pub size_mb: Option<Decimal>,
    pub published_at: Option<chrono::NaiveDateTime>,
    pub duration_seconds: Option<i32>,
    pub resolution: Option<String>,
    pub video_height: Option<String>    
}