use crate::{
    articles::{model::ArticleModel, schema::CreateArticleSchema},
    SharedState,
};
use actix_web::{delete, get, post, put, web, HttpResponse, Responder, Result};
use chrono::Utc;
use log::{error, info};
use uuid::Uuid;

use super::schema::UpdateArticleSchema;

#[post("")]
async fn create_article(
    body: web::Json<CreateArticleSchema>,
    data: web::Data<SharedState>,
) -> Result<impl Responder> {
    let db = data.db.lock().await.clone();
    let datetime = Utc::now().naive_utc();
    match sqlx::query_as!(
        ArticleModel,
        "INSERT INTO articles VALUES($1,$2,$3,$4);",
        Uuid::new_v4(),
        body.title,
        body.article,
        datetime
    )
    .execute(&db)
    .await
    {
        Ok(_) => {
            info!("Article {} created successfully", body.title);
            Ok(HttpResponse::Ok())
        }
        Err(err) => {
            error!("{err}");
            Err(actix_web::error::ErrorInternalServerError(err))
        }
    }
}

#[get("")]
async fn read_articles(data: web::Data<SharedState>) -> Result<impl Responder> {
    let db = data.db.lock().await.clone();
    let query_result = match sqlx::query_as!(ArticleModel, "SELECT * FROM articles")
        .fetch_all(&db)
        .await
    {
        Ok(articles) => articles,
        Err(err) => {
            error!("{err}");
            Vec::new()
        }
    };
    Ok(HttpResponse::Ok().json(query_result))
}

#[put("/{id}")]
async fn update_article(
    id: web::Path<Uuid>,
    body: web::Json<UpdateArticleSchema>,
    data: web::Data<SharedState>,
) -> Result<impl Responder> {
    let db = data.db.lock().await.clone();
    let datetime = Utc::now().naive_utc();
    match sqlx::query_as!(
        ArticleModel,
        "UPDATE articles SET title=$2, article=$3, modified=$4 WHERE id = $1;",
        id.clone(),
        body.title,
        body.article,
        datetime
    )
    .execute(&db)
    .await
    {
        Ok(_) => {
            info!("Article {id} updated successfully");
            Ok(HttpResponse::Ok())
        }
        Err(err) => {
            error!("{err} for {id}");
            Err(actix_web::error::ErrorInternalServerError(err))
        }
    }
}

#[delete("/{id}")]
async fn delete_article(
    id: web::Path<Uuid>,
    data: web::Data<SharedState>,
) -> Result<impl Responder> {
    let db = data.db.lock().await.clone();
    match sqlx::query_as!(
        ArticleModel,
        "DELETE FROM articles WHERE id=$1;",
        id.clone(),
    )
    .execute(&db)
    .await
    {
        Ok(_) => {
            info!("Article {id} deleted successfully");
            Ok(HttpResponse::Ok())
        }
        Err(err) => {
            error!("Deleting article {id} failed: {err} ");
            Err(actix_web::error::ErrorInternalServerError(err))
        }
    }
}

pub fn articles_config(conf: &mut web::ServiceConfig) {
    let scope = web::scope("api/articles")
        .service(create_article)
        .service(read_articles)
        .service(update_article)
        .service(delete_article);
    conf.service(scope);
}
