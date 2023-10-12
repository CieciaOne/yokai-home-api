use crate::{
    common::Db,
    network::model::{OFFLINE, ONLINE},
};

use super::model::DeviceModel;
use anyhow::Result;
use log::{info, warn};
use std::{net::IpAddr, time::Duration};
use tokio::time::interval;

pub struct StatusMonitor {
    db: Db,
    interval: Duration,
}

impl StatusMonitor {
    pub fn new(db: Db, interval: Duration) -> Self {
        Self { db, interval }
    }

    pub async fn run(&self) -> Result<()> {
        info!("Started Network Status Monitor");
        let mut interval = interval(self.interval);
        let payload = [0; 8];

        loop {
            interval.tick().await;

            let db = &self.db.lock().await.clone();
            let devices = sqlx::query_as!(DeviceModel, "SELECT * FROM devices;")
                .fetch_all(db)
                .await?;

            for dev in devices {
                let ip = dev.ip.parse::<IpAddr>()?;
                match surge_ping::ping(ip, &payload).await {
                    Ok(_) => {
                        if !dev.status {
                            self.set_device_state(&dev, ONLINE).await?;
                        }
                    }
                    Err(_) => {
                        if dev.status {
                            self.set_device_state(&dev, OFFLINE).await?;
                        }
                    }
                };
            }
        }
    }

    async fn set_device_state(&self, dev: &DeviceModel, state: bool) -> Result<bool> {
        let db = &self.db.lock().await.clone();
        match sqlx::query_as!(
            DeviceModel,
            "UPDATE devices SET status=$1 WHERE id=$2;",
            state,
            dev.id
        )
        .execute(db)
        .await
        {
            Ok(_) => {
                let state_repr = match state {
                    true => "Online",
                    false => "Offline",
                };
                info!("Device:{} is now {}", dev.id.to_string(), state_repr);
                Ok(state)
            }
            Err(err) => {
                warn!("{err}");
                Err(err.into())
            }
        }
    }
}
