use sqlx::PgPool;
use uuid::Uuid;
use crate::{
    errors::app_error::AppError,
    models::entity::ShortUrl,
};

pub struct ShortenerRepository;

impl ShortenerRepository {
    pub async fn insert(
        pool: &PgPool,
        entity: &ShortUrl,
    ) -> Result<(), AppError> {
        sqlx::query!(
            r#"
            INSERT INTO links (id, original_url, short_code)
            VALUES ($1, $2, $3)
            "#,
            entity.id,
            entity.original_url,
            entity.short_code
        )
            .execute(pool)
            .await
            .map_err(AppError::from)?;

        Ok(())
    }

    #[allow(dead_code)]
    pub async fn update(
        pool: &PgPool,
        id: Uuid,
        original_url: &str,
        short_code: &str,
    ) -> Result<(), AppError> {
        let result = sqlx::query!(
            r#"
            UPDATE links
            SET original_url = $1,
                short_code = $2
            WHERE id = $3
            "#,
            original_url,
            short_code,
            id
        )
            .execute(pool)
            .await
            .map_err(AppError::from)?;

        if result.rows_affected() == 0 {
            return Err(AppError::NotFound);
        }

        Ok(())
    }
}