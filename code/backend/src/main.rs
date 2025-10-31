use actix_web::{App, HttpServer, Responder, post, web};
use dotenvy::dotenv;
use mongodb::{Client, Database, bson::doc};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::env;

pub mod services;

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: String,
    exp: usize,
}

#[derive(Debug, Deserialize)]
struct SignupData {
    username: String,
    password: String,
}

#[post("/v1/public/signup")]
async fn signup(data: web::Json<SignupData>, db: web::Data<Database>) -> impl Responder {
    let res = services::auth::signup(db, &data.username, &data.password).await;

    match res {
        Ok(id) => {
            log::info!("Login successful: {}", id);

            web::Json(json!({
                "id": id,
            }))
        }
        Err(err) => {
            log::info!("Login failed with {:?}", err);

            web::Json(json!({
                "err": err,
            }))
        }
    }
}

#[derive(Debug, Deserialize)]
struct LoginData {
    username: String,
    password: String,
}

#[post("/v1/public/login")]
async fn login(data: web::Json<LoginData>, db: web::Data<Database>) -> impl Responder {
    web::Json("")
}

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
            .service(signup)
            .service(login)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
