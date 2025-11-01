use actix_cors::Cors;
use actix_web::{web, App, HttpServer};
use dotenvy::dotenv;
use log::info;
use mongodb::{
    bson::doc,
    Client,
};
use std::env;

mod handlers;
mod models;
mod services;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    env_logger::init();

    let mongo_uri = env::var("MONGO_URI").expect("MONGO_URI not set");
    let client = Client::with_uri_str(&mongo_uri)
        .await
        .expect("Failed to connect to MongoDB");

    let db_name = "main";
    let db = client.database(db_name);
    let users = db.collection::<mongodb::bson::Document>("users");

    if let Err(e) = ensure_indexes(&users).await {
        eprintln!("⚠️ Failed to ensure MongoDB indexes: {:?}", e);
    } else {
        info!("✅ Ensured MongoDB indexes for 'users' collection");
    }

    let port: u16 = env::var("PORT")
        .map(|p| p.parse().expect("Port must be a valid 16-bit integer"))
        .unwrap_or(8080);

    info!("🚀 App listening on 0.0.0.0:{port}");

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

async fn ensure_indexes(users: &mongodb::Collection<mongodb::bson::Document>) -> mongodb::error::Result<()> {
    use mongodb::options::IndexOptions;

    let index_model = mongodb::IndexModel::builder()
        .keys(doc! { "username": 1 })
        .options(
            IndexOptions::builder()
                .unique(true)
                .name(Some("unique_username".to_string()))
                .build(),
        )
        .build();

    users.create_index(index_model).await?;
    Ok(())
}

