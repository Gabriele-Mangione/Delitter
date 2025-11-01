use actix_web::{
    App, HttpServer,
    web::{self},
};
use dotenvy::dotenv;
use mongodb::Client;
use std::env;

pub mod handlers;
pub mod models;
pub mod services;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    env_logger::init();

    let mongo_uri = env::var("MONGO_URI").expect("MONGO_URI not set");
    let client = Client::with_uri_str(&mongo_uri)
        .await
        .expect("Failed to connect to MongoDB");
    let db_name = "main";
    let db = client.database(&db_name);

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(db.clone()))
            .service(handlers::auth::signin)
            .service(handlers::auth::signup)
            .service(handlers::litter::create_litter)
            .service(handlers::litter::get_litter)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
