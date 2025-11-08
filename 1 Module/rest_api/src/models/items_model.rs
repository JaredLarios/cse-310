use actix_web::{web};
use sqlx::{SqlitePool};
use uuid::Uuid;
use crate::schema::items_schema::Item;
use crate::schema::items_schema::ItemBase;

pub async fn get_items_list(pool: &web::Data<SqlitePool>) -> Result<Vec<Item>, sqlx::Error> {
    let rows = sqlx::query_as("SELECT * FROM items")
        .fetch_all(pool.get_ref())
        .await
        .unwrap();
    Ok(rows)
}

pub async fn get_item_by_uuid(item_uuid: &Uuid, pool: &web::Data<SqlitePool>) -> Result<Item, sqlx::Error> {
    let item = sqlx::query_as::<_, Item>("SELECT * FROM items where uuid = ?1")
        .bind(item_uuid.to_string())
        .fetch_one(pool.get_ref())
        .await?;
    Ok(item)
}

pub async fn add_item(item: ItemBase, pool: &web::Data<SqlitePool>) -> Result<Uuid, sqlx::Error> {
    let new_uuid = Uuid::new_v4();

    sqlx::query("INSERT INTO items (uuid, name, description) VALUES (?1, ?2, ?3)")
        .bind(new_uuid.to_string())
        .bind(&item.name)
        .bind(&item.description)
        .execute(pool.get_ref())
        .await?;
    Ok(new_uuid)
}

pub async fn edit_item(uuid: &Uuid, item: ItemBase, pool: &web::Data<SqlitePool>) -> Result<Uuid, sqlx::Error> {
    sqlx::query("UPDATE items SET name = ?1, description = ?2 WHERE uuid = ?3")
        .bind(&item.name)
        .bind(&item.description)
        .bind(uuid.to_string())
        .execute(pool.get_ref())
        .await?;
    Ok(*uuid)
}

pub async fn remove_item(uuid: &Uuid, pool: &web::Data<SqlitePool>) -> Result<Uuid, sqlx::Error> {
    let result = sqlx::query("DELETE FROM items WHERE uuid = ?1")
        .bind(uuid.to_string())
        .execute(pool.get_ref())
        .await?;

    if result.rows_affected() == 0 {
        // You can return a custom error or handle it upstream
        return Err(sqlx::Error::RowNotFound);
    }

    Ok(*uuid)
}