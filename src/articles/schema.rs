use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateArticleSchema {
    pub title: String,
    pub article: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateArticleSchema {
    pub title: String,
    pub article: String,
}
