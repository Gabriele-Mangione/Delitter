use actix_cors::Cors;
use actix_web::{
    App, HttpServer,
    web::{self},
};
use dotenvy::dotenv;
use log::info;
use mongodb::Client;
use std::env;

pub mod handlers;
pub mod models;
pub mod services;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    env_logger::init();

    env::var("IMAGE_RECOGNITION_URL").expect("IMAGE_RECOGNITION_URL not set");
    let mongo_uri = env::var("MONGO_URI").expect("MONGO_URI not set");
    let client = Client::with_uri_str(&mongo_uri)
        .await
        .expect("Failed to connect to MongoDB");
    let db_name = "main";
    let db = client.database(&db_name);

    let port: u16 = env::var("PORT")
        .map(|p| p.parse().expect("Port must be a valid 16 bit integer"))
        .unwrap_or(8080);

    info!("App listening over 0.0.0.0:{port}");

    HttpServer::new(move || {
        App::new()
            .wrap(Cors::permissive())
            .app_data(web::Data::new(db.clone()))
            .service(handlers::alive)
            .service(handlers::auth::signin)
            .service(handlers::auth::signup)
            .service(handlers::litter::create_litter)
            .service(handlers::litter::get_litter)
    })
    .bind(("0.0.0.0", port))?
    .run()
    .await
}
