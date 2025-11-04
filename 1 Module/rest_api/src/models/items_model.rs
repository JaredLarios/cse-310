use actix_web::{web};
use sqlx::{prelude::FromRow, Executor, SqlitePool};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::schema::items_schema::Item;

pub async fn get_items_list(pool: &web::Data<SqlitePool>) -> Result<Vec<Item>, sqlx::Error> {
    let rows = sqlx::query_as("SELECT * FROM items")
        .fetch_all(pool.get_ref())
        .await
        .unwrap();
    Ok(rows)
}

pub async fn get_item_by_uuid(item_uuid: &Uuid, pool: &web::Data<SqlitePool>) -> Result<Vec<Item>, sqlx::Error> {
    let item = sqlx::query_as("SELECT * FROM items where uuid= ?1")
        .fetch_all(pool.get_ref())
        .await
        .unwrap();
    Ok(item)
}
