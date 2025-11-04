use serde::{Serialize, Deserialize};
use sqlx::{prelude::FromRow};
use uuid::Uuid;

#[derive(Serialize, Deserialize, FromRow)]
pub struct Item {
    pub uuid: String,
    pub name: String,
    pub description: String,
}
