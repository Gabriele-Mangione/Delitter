use actix_web::{HttpResponse, ResponseError, http::StatusCode, web::Json};
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
}

impl ResponseError for HttpError {
    fn status_code(&self) -> actix_web::http::StatusCode {
        match *self {
            Self::InvalidCredentials => StatusCode::UNAUTHORIZED,
            Self::InvalidToken => StatusCode::FORBIDDEN,
        }
    }

    fn error_response(&self) -> actix_web::HttpResponse<actix_web::body::BoxBody> {
        HttpResponse::build(self.status_code()).json(Json(json!({
            "message": self.to_string()
        })))
    }
}
