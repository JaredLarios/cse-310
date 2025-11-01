use actix_web::{web, App, HttpServer, Responder};
mod models;
mod schema;

async fn hello() -> impl Responder {
    "Hello World!"
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/",web::get().to(hello))
            .route("/items",
                web::get().to(models::items_model::get_items))
            .route("/item",
                web::get().to(models::items_model::get_item))
            .route("/item",
                web::post().to(models::items_model::create_item))
            .route("/item",
                web::put().to(models::items_model::update_item))
            .route("/item",
                web::delete().to(models::items_model::delete_item))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
