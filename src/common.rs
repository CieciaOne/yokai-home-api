use serde::{Deserialize, Serialize};
use std::{collections::HashMap, sync::Arc};
use tokio::sync::Mutex;

use sqlx::{Pool, Postgres};
use uuid::Uuid;

pub type Db = Arc<Mutex<Pool<Postgres>>>;

#[derive(Serialize, Deserialize)]
pub struct Channel {
    name: String,
    link: String,
    items: Items,
}

impl Channel {
    pub fn new(name: String, link: String, items: Items) -> Self {
        Self { name, link, items }
    }
}

pub type Items = Vec<Item>;

#[derive(Serialize, Deserialize)]
pub struct Item {
    title: Option<String>,
    link: Option<String>,
    description: Option<String>,
    author: Option<String>,
}

impl Item {
    pub fn new(
        title: Option<String>,
        link: Option<String>,
        description: Option<String>,
        author: Option<String>,
    ) -> Self {
        Self {
            title,
            link,
            description,
            author,
        }
    }
}
pub type ChannelMap = HashMap<Uuid, Channel>;
