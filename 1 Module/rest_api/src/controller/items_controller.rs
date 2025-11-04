use actix_web::{web, HttpResponse};
use serde::{Deserialize};
use uuid::Uuid;
use serde_json::json;
use crate::schema::items_schema::Item;
use crate::models::items_model::{get_items_list, get_item_by_uuid};
use sqlx::{SqlitePool};

#[derive(Deserialize)]
pub struct ItemQuery {
    pub item_id: Uuid,
}

pub async fn create_item(item: web::Json<Item>, pool: web::Data<SqlitePool>) -> HttpResponse {
    HttpResponse::Ok().json(item.into_inner())
}

pub async fn get_items(pool: web::Data<SqlitePool>) -> HttpResponse {
    let items = get_items_list(&pool).await.unwrap();
    HttpResponse::Ok().json(items)
}

pub async fn get_item(query: web::Query<ItemQuery>, pool: web::Data<SqlitePool>) -> HttpResponse {
    let item = get_item_by_uuid(&query.item_id,&pool).await.unwrap();
    HttpResponse::Ok().json(item)
}

pub async fn update_item(
    query: web::Query<ItemQuery>,item: web::Json<Item>,
    pool: web::Data<SqlitePool>
) -> HttpResponse {
    let mut update_item = item;
    update_item.uuid = query.item_id.to_string();
    HttpResponse::Ok().json(update_item)
}

pub async fn delete_item(query: web::Query<ItemQuery>, pool: web::Data<SqlitePool>) -> HttpResponse {
    HttpResponse::Ok().json(json!({ "delete": query.item_id }))
}