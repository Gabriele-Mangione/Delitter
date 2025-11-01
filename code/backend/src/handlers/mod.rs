use actix_web::{
    HttpResponse, Responder, ResponseError, get,
    http::StatusCode,
    web::{self, Json},
};
use serde_json::json;
use utoipa::ToSchema;

use derive_more::derive::{Display, Error};
pub mod auth;
pub mod litter;

#[derive(Debug, Serialize, ToSchema)]
pub struct ErrorResponse {
    message: String,
}

use serde::Serialize;

#[derive(Debug, Display, Error)]
pub enum HttpError {
    #[display("Invalid Credentials")]
    InvalidCredentials,
    #[display("Invalid Token")]
    InvalidToken,
    #[display("The provided username already exists")]
    UserAlreadyExists,
    #[display("Network error")]
    NetworkError,
}

impl ResponseError for HttpError {
    fn status_code(&self) -> actix_web::http::StatusCode {
        match *self {
            Self::InvalidCredentials => StatusCode::UNAUTHORIZED,
            Self::InvalidToken => StatusCode::FORBIDDEN,
            Self::UserAlreadyExists => StatusCode::CONFLICT,
            Self::NetworkError => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    fn error_response(&self) -> actix_web::HttpResponse<actix_web::body::BoxBody> {
        HttpResponse::build(self.status_code()).json(Json(json!({
            "message": self.to_string()
        })))
    }
}

#[get("/")]
pub async fn root_redirect() -> impl Responder {
    HttpResponse::Found()
        .append_header(("Location", "/docs/"))
        .finish()
}

#[utoipa::path(
    get,
    path = "/v1/alive",
    responses(
        (status = 200, description = "Server is alive")
    ),
    tag = "Health"
)]
#[get("/v1/alive")]
pub async fn alive() -> impl Responder {
    web::Html::new("OK".to_string())
}
