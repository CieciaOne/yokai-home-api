use serde::{Deserialize, Serialize};
use uuid::Uuid;
#[derive(Debug, Serialize, Deserialize)]
#[allow(non_snake_case)]
pub struct ChannelModel {
    pub id: Uuid,
    pub name: String,
    pub url: String,
}
