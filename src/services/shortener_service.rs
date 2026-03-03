use uuid::Uuid;
use nanoid::nanoid;
use sqlx::PgPool;
use crate::{
    models::{
        requests::CreateShortUrlRequest,
        response::CreateShortUrlResponse,
        entity::ShortUrl,
    },
    repositories::shortener_repo::ShortenerRepository,
    errors::app_error::AppError,
};

pub struct ShortenerService;

impl ShortenerService {
    pub async fn create(
        pool: &PgPool,
        payload: CreateShortUrlRequest,
    ) -> Result<CreateShortUrlResponse, AppError> {
        if payload.url.is_empty() {
            return Err(AppError::Validation);
        }
        
        let code = payload
            .short_code
            .unwrap_or_else(|| nanoid!(8));
        
        let entity = ShortUrl {
            id: Uuid::new_v4(),
            original_url: payload.url.clone(),
            short_code: code.clone(),
        };
        
        ShortenerRepository::insert(pool, &entity).await?;
        
        Ok(CreateShortUrlResponse {
            url: format!("http://localhost:3000/{}", code),
            short_code: code,
        })
    }
}