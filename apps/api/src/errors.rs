use axum::{
    extract::rejection::{FormRejection, JsonRejection},
    http::StatusCode,
    response::IntoResponse,
};
use axum_macros::FromRequest;
use diesel_async::pooled_connection::PoolError;
use serde_json::json;
use std::net::AddrParseError;
use thiserror::Error;

#[derive(FromRequest)]
#[from_request(via(axum::Json), rejection(AppError))]
pub struct Json<T>(T);

#[derive(Error, Debug)]
pub enum AppError {
    #[error("User with username '{0}' not found")]
    UsernameNotFound(String),

    #[error("User with id '{0}' not found")]
    UserNotFound(String),

    #[error("'{}' is not a valid uuid", .0)]
    InvalidUuid(String),

    #[error("Invalid username or password")]
    InvalidUsernameOrPassword,

    #[error("User password not found, please contact admin to reset your password")]
    UserPasswordNotFound,

    #[error("Validation error: {:?}",.0.to_string().replace('\n', ", "))]
    ValidationError(#[from] validator::ValidationErrors),

    #[error("Error parsing token")]
    TokenParseError,

    #[error("Invalid token")]
    InvalidToken,

    #[error("Invalid access token")]
    InvalidAccessToken,

    #[error("Invalid refresh token")]
    InvalidRefreshToken,

    #[error("Token payload error")]
    TokenPayloadError,

    #[error("Token not found, please login")]
    TokenNotFound,

    #[error("Form error: {0}")]
    AxumFormRejection(#[from] FormRejection),

    #[error("Body Json error: {0}")]
    AxumJsonRejection(#[from] JsonRejection),

    #[error("Unsupported media type")]
    UnsupportedMediaType,

    #[error("Internal error")]
    InternalError,

    #[error("Error parsing env variable")]
    EnvError(#[from] envy::Error),

    #[error("Error connecting to db")]
    DbError(#[from] PoolError),

    #[error("Error binding address")]
    AddrError(#[from] AddrParseError),
}

impl AppError {
    pub fn status_code(&self) -> StatusCode {
        match self {
            AppError::UsernameNotFound(_) => StatusCode::NOT_FOUND,
            AppError::UserNotFound(_) => StatusCode::NOT_FOUND,
            AppError::InvalidUuid(_) => StatusCode::BAD_REQUEST,
            AppError::InvalidUsernameOrPassword => StatusCode::UNAUTHORIZED,
            AppError::UserPasswordNotFound => StatusCode::UNAUTHORIZED,
            AppError::ValidationError(_) => StatusCode::BAD_REQUEST,
            AppError::TokenParseError => StatusCode::BAD_REQUEST,
            AppError::InvalidToken => StatusCode::UNAUTHORIZED,
            AppError::InvalidAccessToken => StatusCode::UNAUTHORIZED,
            AppError::InvalidRefreshToken => StatusCode::UNAUTHORIZED,
            AppError::TokenPayloadError => StatusCode::UNAUTHORIZED,
            AppError::TokenNotFound => StatusCode::UNAUTHORIZED,
            AppError::AxumFormRejection(_) => StatusCode::BAD_REQUEST,
            AppError::AxumJsonRejection(_) => StatusCode::BAD_REQUEST,
            AppError::UnsupportedMediaType => StatusCode::UNSUPPORTED_MEDIA_TYPE,
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        let payload = json!({
            "message": self.to_string(),
        });

        (self.status_code(), axum::Json(payload)).into_response()
    }
}
