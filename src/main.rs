mod folder;
mod models;
mod auth;
mod jwt_middleware;

use folder::handlers;
use actix_web::{ web, App, HttpServer, middleware::Logger };
use mongodb::{ sync::Client, options::ClientOptions };
use std::env;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenvy::dotenv().ok();
    env_logger::init();

    let mongo_uri = env::var("MONGO_URI").expect("MONGO_URI must be set");
    let client_options = ClientOptions::parse(&mongo_uri).await.unwrap();
    let mongo_client = Client::with_options(client_options).unwrap();

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .app_data(web::Data::new(mongo_client.clone()))
            .service(handlers::register)
            .service(handlers::login)
            .service(handlers::protected)
    })
        .bind("127.0.0.1:8080")?
        .run().await
}
