use serde::{Serialize, Deserialize};
use sqlx::{prelude::FromRow};
use utoipa::ToSchema;

#[derive(Serialize, Deserialize, FromRow, ToSchema)]
pub struct Item {
    pub uuid: String,
    pub name: String,
    pub description: String,
}

#[derive(Serialize, Deserialize, ToSchema)]
pub struct ItemBase {
    #[schema(example = "Boats", required = true)]
    pub name: String,
    #[schema(example = "Thing that float on the sea", required = true)]
    pub description: String,
}
