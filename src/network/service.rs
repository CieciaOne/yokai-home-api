use std::str::FromStr;

use crate::{
    network::{model::DeviceModel, schema::CreateDeviceSchema},
    SharedState,
};
use actix_web::{error::ErrorInternalServerError,delete, get, post, put, web, HttpResponse, Responder, Result};
use log::{error, info};
use uuid::Uuid;
use wol::MacAddr;

use super::schema::UpdateDeviceSchema;

#[post("")]
async fn create_device(
    body: web::Json<CreateDeviceSchema>,
    data: web::Data<SharedState>,
) -> Result<impl Responder> {
    let db = data.db.lock().await.clone();
    match sqlx::query_as!(
        DeviceModel,
        "INSERT INTO devices VALUES($1,$2,$3,$4,$5);",
        Uuid::new_v4(),
        body.name,
        body.ip,
        body.mac,
        false // default value for status
    )
    .execute(&db)
    .await
    {
        Ok(_) => {
            info!("Device {} created successfully", body.name);
            Ok(HttpResponse::Ok())
        }
        Err(err) => {
            error!("{err}");
            Err(actix_web::error::ErrorInternalServerError(err))
        }
    }
}

#[get("")]
async fn read_devices(data: web::Data<SharedState>) -> Result<impl Responder> {
    let db = data.db.lock().await.clone();
    let query_result = match sqlx::query_as!(DeviceModel, "SELECT * FROM devices")
        .fetch_all(&db)
        .await
    {
        Ok(devices) => devices,
        Err(err) => {
            error!("{err}");
            Vec::new()
        }
    };
    Ok(HttpResponse::Ok().json(query_result))
}

#[put("/{id}")]
async fn update_device(
    id: web::Path<Uuid>,
    body: web::Json<UpdateDeviceSchema>,
    data: web::Data<SharedState>,
) -> Result<impl Responder> {
    let db = data.db.lock().await.clone();
    match sqlx::query_as!(
        DeviceModel,
        "UPDATE devices SET name=$2, ip=$3, mac=$4 WHERE id = $1;",
        id.clone(),
        body.name,
        body.ip,
        body.mac,
    )
    .execute(&db)
    .await
    {
        Ok(_) => {
            info!("Device {id} updated successfully");
            Ok(HttpResponse::Ok())
        }
        Err(err) => {
            error!("{err} for {id}");
            Err(actix_web::error::ErrorInternalServerError(err))
        }
    }
}

#[delete("/{id}")]
async fn delete_device(
    id: web::Path<Uuid>,
    data: web::Data<SharedState>,
) -> Result<impl Responder> {
    let db = data.db.lock().await.clone();
    match sqlx::query_as!(DeviceModel, "DELETE FROM devices WHERE id=$1;", id.clone())
        .execute(&db)
        .await
    {
        Ok(_) => {
            info!("Device {id} deleted successfully");
            Ok(HttpResponse::Ok())
        }
        Err(err) => {
            error!("Deleting device {id} failed: {err} ");
            Err(actix_web::error::ErrorInternalServerError(err))
        }
    }
}

#[get("wol/{id}")]
async fn wake_device(id: web::Path<Uuid>, data: web::Data<SharedState>) -> Result<impl Responder> {
    let db = data.db.lock().await.clone();
    match sqlx::query_as!(DeviceModel, "SELECT * FROM devices WHERE id=$1", id.clone())
        .fetch_one(&db)
        .await
    {
        Ok(device) => {
            let mac = MacAddr::from_str(&device.mac)
                .map_err(ErrorInternalServerError)?;
            wol::send_wol(mac, None, None)
                .map_err(ErrorInternalServerError)?;

            Ok(HttpResponse::Ok().json(device))
        }
        Err(err) => {
            error!("{err}");
            Err(actix_web::error::ErrorInternalServerError(err))
        }
    }
}

pub fn network_config(conf: &mut web::ServiceConfig) {
    let scope = web::scope("api/network")
        .service(create_device)
        .service(read_devices)
        .service(update_device)
        .service(delete_device)
        .service(wake_device);

    conf.service(scope);
}
