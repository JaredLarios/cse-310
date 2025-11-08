use actix_web::{web, App, HttpServer, Responder, middleware::Logger};
mod controller;
mod models;
mod schema;
mod utils;

use utoipa::{OpenApi};
use utoipa_swagger_ui::SwaggerUi;

async fn hello() -> impl Responder {
    "Hello World!"
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    #[derive(OpenApi)]
    #[openapi(
        paths(
            controller::items_controller::get_items,
            controller::items_controller::get_item,
            controller::items_controller::create_item,
            controller::items_controller::update_item,
            controller::items_controller::delete_item
        ),
        components(
            schemas(
                schema::items_schema::Item,
                schema::items_schema::ItemBase
            )
        ),
        tags(
            (name = "Items", description = "Item management endpoints")
        )
    )]
    struct MyApiDoc;

    let openapi = MyApiDoc::openapi();

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
            .service(SwaggerUi::new("/docs/{_:.*}")
                .url("/api-docs/openapi.json", openapi.clone()))
            .route("/",web::get().to(hello))
            .service(controller::items_controller::get_items)
            .service(controller::items_controller::get_item)
            .service(controller::items_controller::create_item)
            .service(controller::items_controller::update_item)
            .service(controller::items_controller::delete_item)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
