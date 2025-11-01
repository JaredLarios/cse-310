use actix_web::{web, HttpResponse};
use serde::{Deserialize};
use uuid::Uuid;
use serde_json::json;
use crate::schema::items_schema::Item;

#[derive(Deserialize)]
pub struct ItemQuery {
    pub item_id: Uuid,
}

pub async fn create_item(item: web::Json<Item>) -> HttpResponse {
    HttpResponse::Ok().json(item.into_inner())
}

pub async fn get_items() -> HttpResponse {
    let items = vec![
        Item {
            id: Uuid::new_v4(),
            name: String::from("Item 1"),
            description: String::from("Description 1"),
        },
        Item {
            id: Uuid::new_v4(),
            name: String::from("Item 2"),
            description: String::from("Description 2"),
        },
    ];
    HttpResponse::Ok().json(items)
}

pub async fn get_item(query: web::Query<ItemQuery>) -> HttpResponse {
    let item = Item {
            id: query.item_id,
            name: String::from("Item 1"),
            description: String::from("Description 1"),
        };
    HttpResponse::Ok().json(item)
}

pub async fn update_item(
    query: web::Query<ItemQuery>,item: web::Json<Item>) -> HttpResponse {
    let mut update_item = item;
    update_item.id = query.item_id;
    HttpResponse::Ok().json(update_item)
}

pub async fn delete_item(query: web::Query<ItemQuery>) -> HttpResponse {
    HttpResponse::Ok().json(json!({ "delete": query.item_id }))
}