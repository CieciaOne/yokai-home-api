use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
#[derive(Debug, Serialize, Deserialize)]
#[allow(non_snake_case)]
pub struct ArticleModel {
    pub id: Uuid,
    pub title: String,
    pub article: Option<String>,
    pub modified: NaiveDateTime,
}
