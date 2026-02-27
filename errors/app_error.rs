use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use thiserror::Error;
use crate::models::response::ApiResponse;
#[derive(Error, Debug)]
pub enum AppError {
    #[error("invalid input")]
    Validation,
    #[error("not found")]
    NotFound,
    #[error("conflict")]
    Conflict,
    #[error("database error")]
    Database(#[from] sqlx::Error),
    #[error("internal error")]
    Internal,
}
impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, code, message) = match self {
            AppError::Validation =>
                (StatusCode::BAD_REQUEST, "VALIDATION", "invalid input"),
            AppError::NotFound =>
                (StatusCode::NOT_FOUND, "NOT_FOUND", "not found"),
            AppError::Conflict =>
                (StatusCode::CONFLICT, "CONFLICT", "already exists"),
            AppError::Database(_) =>
                (StatusCode::INTERNAL_SERVER_ERROR, "INTERNAL", "internal error"),
            AppError::Internal =>
                (StatusCode::INTERNAL_SERVER_ERROR, "INTERNAL", "internal error"),
        };
        let body = ApiResponse::<()>::error(
            code.into(),
            message.into(),
        );
        (status, Json(body)).into_response()
    }
}