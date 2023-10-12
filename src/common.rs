use serde::{Deserialize, Serialize};
use std::{collections::HashMap, sync::Arc};
use tokio::sync::Mutex;

use sqlx::{Pool, Postgres};
use uuid::Uuid;

pub type Db = Arc<Mutex<Pool<Postgres>>>;
pub type ChannelMap = HashMap<Uuid, Channel>;
pub type Store = Arc<Mutex<ChannelMap>>;

#[derive(Serialize, Deserialize, Debug)]
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

#[derive(Serialize, Deserialize, Debug)]
pub struct Item {
    title: Option<String>,
    link: Option<String>,
}

impl Item {
    pub fn new(title: Option<String>, link: Option<String>) -> Self {
        Self { title, link }
    }
}
