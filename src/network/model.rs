use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
#[allow(non_snake_case)]
pub struct DeviceModel {
    pub id: Uuid,
    pub name: String,
    pub ip: String,
    pub mac: String,
    pub status: bool,
}
pub const ONLINE: bool = true;
pub const OFFLINE: bool = false;
