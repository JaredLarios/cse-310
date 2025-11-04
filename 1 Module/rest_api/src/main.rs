use actix_web::{web, App, HttpServer, Responder, middleware::Logger};
mod controller;
mod models;
mod schema;
mod utils;

async fn hello() -> impl Responder {
    "Hello World!"
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Initialize the environment logger
    env_logger::Builder::from_env(
        env_logger::Env::default().default_filter_or("info")
    ).init();

    let pool = utils::database::db().await.unwrap();
    utils::database::create_items_table(&pool).await;
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .wrap(Logger::new("%a %r %s %b %{Referer}i %{User-Agent}i %T"))
            .route("/",web::get().to(hello))
            .route("/items",
                web::get().to(controller::items_controller::get_items))
            .route("/item",
                web::get().to(controller::items_controller::get_item))
            .route("/item",
                web::post().to(controller::items_controller::create_item))
            .route("/item",
                web::put().to(controller::items_controller::update_item))
            .route("/item",
                web::delete().to(controller::items_controller::delete_item))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
