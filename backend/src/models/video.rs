use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;
use rust_decimal::Decimal;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Video {
    pub id: Uuid,
    pub title: String,
    pub file_extension: Option<String>,
    pub file_size_mb: Option<Decimal>,
    pub created_at: Option<chrono::NaiveDateTime>,
}
