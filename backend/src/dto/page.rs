use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct PageQuery {
    pub page: Option<i64>,
    pub size: Option<i64>,
}

#[derive(Debug, Serialize)]
pub struct PageResponse<T>
where
    T: Serialize,
{
    pub items: Vec<T>,
    pub total: i64,
    pub page: i64,
    pub size: i64,
}