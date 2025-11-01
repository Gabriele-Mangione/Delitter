use actix_web::{
    HttpResponse, Responder, ResponseError, get,
    http::StatusCode,
    post,
    web::{self, Json},
};
use serde_json::json;

use derive_more::derive::{Display, Error};
pub mod auth;
pub mod litter;

#[derive(Debug, Display, Error)]
pub enum HttpError {
    #[display("Invalid Credentials")]
    InvalidCredentials,
    #[display("Invalid Token")]
    InvalidToken,
    #[display("The provided username already exists")]
    UserAlreadyExists,
}

impl ResponseError for HttpError {
    fn status_code(&self) -> actix_web::http::StatusCode {
        match *self {
            Self::InvalidCredentials => StatusCode::UNAUTHORIZED,
            Self::InvalidToken => StatusCode::FORBIDDEN,
            Self::UserAlreadyExists => StatusCode::CONFLICT,
        }
    }

    fn error_response(&self) -> actix_web::HttpResponse<actix_web::body::BoxBody> {
        HttpResponse::build(self.status_code()).json(Json(json!({
            "message": self.to_string()
        })))
    }
}

#[get("/v1/alive")]
pub async fn alive() -> impl Responder {
    web::Html::new("OK".to_string())
}
