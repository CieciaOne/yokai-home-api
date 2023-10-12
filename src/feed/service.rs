use crate::{common::Channel, feed::schema::CreateChannelSchema, SharedState};
use actix_web::{
    delete, error::ErrorInternalServerError, get, post, web, HttpResponse, Responder, Result,
};
use log::{error, info};
use uuid::Uuid;

#[get("{id}")]
async fn read_one(id: web::Path<Uuid>, data: web::Data<SharedState>) -> Result<impl Responder> {
    let store = data.store.lock().await;

    match store.get(&id) {
        Some(channel) => Ok(HttpResponse::Ok().json(channel)),
        None => Err(ErrorInternalServerError("err")),
    }
}

#[get("")]
async fn read_feed(data: web::Data<SharedState>) -> Result<impl Responder> {
    let store = data.store.lock().await;

    let res: Vec<&Channel> = store.values().collect();

    Ok(HttpResponse::Ok().json(res))
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
