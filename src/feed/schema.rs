use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateChannelSchema {
    pub name: String,
    pub url: String,
}
