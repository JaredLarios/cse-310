use actix_web::{web, HttpResponse};
use serde::{Deserialize};
use uuid::Uuid;
use serde_json::json;
use crate::schema::items_schema::Item;
use crate::schema::items_schema::ItemBase;
use crate::models::items_model::{get_items_list, get_item_by_uuid, add_item, edit_item};
use sqlx::{SqlitePool};
use std::collections::HashMap;

#[derive(Deserialize)]
pub struct ItemQuery {
    pub item_id: Uuid,
}

pub async fn get_items(pool: web::Data<SqlitePool>) -> HttpResponse {
    let items = get_items_list(&pool).await.unwrap();
    HttpResponse::Ok().json(items)
}

pub async fn get_item(query: web::Query<ItemQuery>, pool: web::Data<SqlitePool>) -> HttpResponse {
    match get_item_by_uuid(&query.item_id,&pool).await {
        Ok(item) => HttpResponse::Ok().json(item),
        Err(sqlx::Error::RowNotFound) => HttpResponse::NotFound().body("Item not found"),
        Err(err) => {
            eprintln!("Database error: {:?}", err);
            HttpResponse::InternalServerError().body("Internal server error")
        }
    }
}

pub async fn create_item(item: web::Json<ItemBase>, pool: web::Data<SqlitePool>) -> HttpResponse {
    let result = add_item(item.into_inner(), &pool).await;

    match result {
        Ok(uuid) => {
            let mut response = HashMap::new();
            response.insert(
                "message".to_string(),
                format!("Item created with UUID: {}", uuid),
            );
            HttpResponse::Ok().json(response)
        }
        Err(err) => {
            eprintln!("Database error: {:?}", err);
            HttpResponse::InternalServerError().finish()
        }
    }
}

pub async fn update_item(
    query: web::Query<ItemQuery>,item: web::Json<ItemBase>,
    pool: web::Data<SqlitePool>
) -> HttpResponse {
    let conf_item = get_item_by_uuid(&query.item_id,&pool).await;
    let mut response = HashMap::new();

    match conf_item {
        Err(sqlx::Error::RowNotFound) => {
            response.insert(
                    "message".to_string(),
                    format!("No item uuid : {} Found", query.item_id),
                );
            return HttpResponse::NotFound().json(response)
        }
        Err(err) => {
            eprintln!("Database error: {:?}", err);
            return HttpResponse::InternalServerError().finish()
        }
        Ok(item) => ()
    }

    match edit_item(&query.item_id, item.into_inner(), &pool).await {
        Ok(conf) => {
            response.insert(
                    "message".to_string(),
                    String::from("Item updated"),
                );
            return HttpResponse::Ok().json(response)
        }
        Err(err) => {
            eprintln!("Database error: {:?}", err);
            return HttpResponse::InternalServerError().finish()
        }
    }

}

pub async fn delete_item(query: web::Query<ItemQuery>, pool: web::Data<SqlitePool>) -> HttpResponse {
    HttpResponse::Ok().json(json!({ "delete": query.item_id }))
}