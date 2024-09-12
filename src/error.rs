#[cfg(feature = "ssr")]
use actix_web::{
    error::ResponseError,
    http::{header::ContentType, StatusCode},
    HttpResponse,
};

#[cfg(feature = "ssr")]
use serde::Serialize;

#[cfg(feature = "ssr")]
#[derive(Serialize, Debug)]
pub struct ErrorResponse {
    status_code: u16,
    error: String,
}
#[cfg(feature = "ssr")]
impl ErrorResponse {
    pub fn new(status_code: u16, message: String) -> Self {
        Self {
            status_code,
            error: message,
        }
    }
}
#[cfg(feature = "ssr")]
#[derive(thiserror::Error, Debug)]
pub enum AppError {
    #[cfg(feature = "ssr")]
    #[error("DatabaseError: {0}")]
    DatabaseError(surrealdb::Error),

    #[allow(dead_code)]
    #[error("{0}")]
    BadRequest(String),

    #[error("IOError: {0}")]
    IOError(std::io::Error),

    #[allow(dead_code)]
    #[error("InternalServerError: {0}")]
    InternalError(String),

    #[allow(dead_code)]
    #[error("Unauthorized")]
    Unauthorized,

    #[allow(dead_code)]
    #[error("Forbidden: {0}")]
    Forbidden(&'static str),

    #[cfg(feature = "ssr")]
    #[allow(dead_code)]
    #[error("Unauthorized: {0}")]
    IdentityError(actix_identity::error::GetIdentityError),

    #[cfg(feature = "ssr")]
    #[allow(dead_code)]
    #[error("Error Creating Session: {0}")]
    SessionInsertError(actix_session::SessionInsertError),
}
#[cfg(feature = "ssr")]
impl ResponseError for AppError {
    fn error_response(&self) -> HttpResponse {
        let status_code = self.status_code().as_u16();
        let error_message = self.to_string();
        let error_response = ErrorResponse::new(status_code, error_message);

        HttpResponse::build(self.status_code())
            .insert_header(ContentType::json())
            .json(error_response)
    }
    fn status_code(&self) -> StatusCode {
        match self {
            #[cfg(feature = "ssr")]
            AppError::DatabaseError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            AppError::IOError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            AppError::InternalError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            AppError::BadRequest(_) => StatusCode::BAD_REQUEST,
            AppError::Unauthorized => StatusCode::UNAUTHORIZED,
            #[cfg(feature = "ssr")]
            AppError::IdentityError(_) => StatusCode::UNAUTHORIZED,
            AppError::Forbidden(_) => StatusCode::FORBIDDEN,
            #[cfg(feature = "ssr")]
            AppError::SessionInsertError(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

#[cfg(feature = "ssr")]
impl From<surrealdb::Error> for AppError {
    fn from(value: surrealdb::Error) -> Self {
        AppError::DatabaseError(value)
    }
}
#[cfg(feature = "ssr")]
impl From<std::io::Error> for AppError {
    fn from(value: std::io::Error) -> Self {
        AppError::IOError(value)
    }
}

#[cfg(feature = "ssr")]
impl From<actix_identity::error::GetIdentityError> for AppError {
    fn from(value: actix_identity::error::GetIdentityError) -> Self {
        AppError::IdentityError(value)
    }
}

#[cfg(feature = "ssr")]
impl From<actix_session::SessionInsertError> for AppError {
    fn from(value: actix_session::SessionInsertError) -> Self {
        AppError::SessionInsertError(value)
    }
}

#[cfg(feature = "ssr")]
impl From<actix_identity::error::LoginError> for AppError {
    fn from(_: actix_identity::error::LoginError) -> Self {
        AppError::Unauthorized
    }
}