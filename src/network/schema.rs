use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateDeviceSchema {
    pub name: String,
    pub ip: String,
    pub mac: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateDeviceSchema {
    pub name: String,
    pub ip: String,
    pub mac: String,
}
