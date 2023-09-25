mod articles;
mod common;
mod feed;
mod network;
use core::result::Result::{Err, Ok};

use crate::articles::service::articles_config;
use crate::feed::service::feed_config;
use crate::network::service::network_config;
use crate::network::status_monitor::StatusMonitor;
use actix_web::{middleware::Logger, web::Data, App, HttpServer};
use anyhow::Result;
use common::Db;
use dotenv::dotenv;
use log::{error, info};
use sqlx::postgres::PgPoolOptions;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::Mutex;

#[derive(Clone)]
pub struct SharedState {
    db: Db,
}

#[tokio::main]
async fn main() -> Result<()> {
    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "info");
    }
    dotenv().ok();
    env_logger::init();

    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let network_scan_interval = Duration::from_secs(
        std::env::var("NETWORK_SCAN_INTERVAL")
            .expect("NETWORK_SCAN_INTERVAL must be set(unit is seconds)")
            .parse::<u64>()?,
    );

    let pool = match PgPoolOptions::new()
        .max_connections(3)
        .connect(&database_url)
        .await
    {
        Ok(pool) => {
            info!("Connection to db succeded!");
            pool
        }
        Err(err) => {
            error!("Connection to DB failed: {}", err);
            std::process::exit(1)
        }
    };
    let pool = Arc::new(Mutex::new(pool));

    let status_pool = pool.clone();
    tokio::spawn(async move {
        StatusMonitor::new(status_pool, network_scan_interval)
            .run()
            .await?;
        anyhow::Ok(())
    });

    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(SharedState { db: pool.clone() }))
            .configure(network_config)
            .configure(articles_config)
            .configure(feed_config)
            .wrap(Logger::default())
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await?;

    Ok(())
}
