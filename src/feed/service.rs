use std::collections::HashMap;

use crate::{
    common::ChannelMap,
    feed::{model::ChannelModel, schema::CreateChannelSchema},
    SharedState,
};
use actix_web::{
    delete, error::ErrorInternalServerError, get, post, web, HttpResponse, Responder, Result,
};
use log::{error, info};
use rss::Channel;
use uuid::Uuid;

#[get("{id}")]
async fn read_one(id: web::Path<Uuid>, data: web::Data<SharedState>) -> Result<impl Responder> {
    let db = &data.db.lock().await.clone();
    let channel = sqlx::query_as!(
        ChannelModel,
        "SELECT * FROM channels WHERE id=$1",
        id.clone()
    )
    .fetch_one(db)
    .await
    .map_err(|err| ErrorInternalServerError(err))?;

    let mut store: ChannelMap = HashMap::new();
    let channel_update = reqwest::get(channel.url)
        .await
        .map_err(|err| ErrorInternalServerError(err))?
        .bytes()
        .await
        .map_err(|err| ErrorInternalServerError(err))?;
    let new_channel =
        Channel::read_from(&channel_update[..]).map_err(|err| ErrorInternalServerError(err))?;
    store.insert(channel.id, new_channel);
    Ok(HttpResponse::Ok().json(store))
}

#[get("")]
async fn read_feed(data: web::Data<SharedState>) -> Result<impl Responder> {
    let db = &data.db.lock().await.clone();
    match sqlx::query_as!(ChannelModel, "SELECT * FROM channels")
        .fetch_all(db)
        .await
    {
        Ok(db_channels_result) => {
            let mut store: ChannelMap = HashMap::new();
            for channel in db_channels_result {
                let channel_update = reqwest::get(channel.url)
                    .await
                    .map_err(|err| ErrorInternalServerError(err))?
                    .bytes()
                    .await
                    .map_err(|err| ErrorInternalServerError(err))?;
                let new_channel = Channel::read_from(&channel_update[..])
                    .map_err(|err| ErrorInternalServerError(err))?;
                store.insert(channel.id, new_channel);
            }
            Ok(HttpResponse::Ok().json(store))
        }
        Err(err) => {
            error!("Reading channels failed: {} ", err);
            Err(actix_web::error::ErrorInternalServerError(err))
        }
    }
}

#[post("")]
async fn create_channel(
    body: web::Json<CreateChannelSchema>,
    data: web::Data<SharedState>,
) -> Result<impl Responder> {
    let db = &data.db.lock().await.clone();
    match sqlx::query_as!(
        ChannelModel,
        "INSERT INTO channels VALUES($1,$2,$3);",
        Uuid::new_v4(),
        body.name,
        body.url,
    )
    .execute(db)
    .await
    {
        Ok(_) => {
            info!("Channel {} created successfully", body.name);
            Ok(HttpResponse::Ok())
        }
        Err(err) => {
            error!("Creating channel {} failed: {} ", body.name, err);
            Err(actix_web::error::ErrorInternalServerError(err))
        }
    }
}

#[delete("/{id}")]
async fn delete_channel(
    id: web::Path<Uuid>,
    data: web::Data<SharedState>,
) -> Result<impl Responder> {
    let db = &data.db.lock().await.clone();
    match sqlx::query_as!(
        ChannelModel,
        "DELETE FROM channels WHERE id=$1;",
        id.clone()
    )
    .execute(db)
    .await
    {
        Ok(_) => {
            info!("Channel {id} deleted successfully");
            Ok(HttpResponse::Ok())
        }
        Err(err) => {
            error!("Deleting channel {id} failed: {err} ");
            Err(actix_web::error::ErrorInternalServerError(err))
        }
    }
}

pub fn feed_config(conf: &mut web::ServiceConfig) {
    let channel_scope = web::scope("api/channel")
        .service(create_channel)
        .service(delete_channel);
    let feed_scope = web::scope("/api/feed").service(read_feed).service(read_one);

    conf.service(channel_scope).service(feed_scope);
}
