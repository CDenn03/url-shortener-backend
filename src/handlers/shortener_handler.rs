use axum::{extract::State, Json};
use sqlx::PgPool;
use crate::{
    models::{
        requests::CreateShortUrlRequest,
        response::{ApiResponse, CreateShortUrlResponse},
    },
    services::shortener_service::ShortenerService,
    errors::app_error::AppError,
};

pub async fn create_short_url(
    State(pool): State<PgPool>,
    Json(payload): Json<CreateShortUrlRequest>,
) -> Result<Json<ApiResponse<CreateShortUrlResponse>>, AppError> {
    let result = ShortenerService::create(&pool, payload).await?;
    Ok(Json(ApiResponse::success(result))) 
}