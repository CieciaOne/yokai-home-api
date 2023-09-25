use std::{collections::HashMap, sync::Arc};
use tokio::sync::Mutex;

use rss::Channel;
use sqlx::{Pool, Postgres};
use uuid::Uuid;

pub type Db = Arc<Mutex<Pool<Postgres>>>;

pub type ChannelMap = HashMap<Uuid, Channel>;
