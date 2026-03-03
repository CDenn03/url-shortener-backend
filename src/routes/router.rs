use axum::{Router, routing::post};
use sqlx::PgPool;
use crate::handlers::shortener_handler::create_short_url;

pub fn router() -> Router<PgPool> {
    Router::new()
        .route("/shorten", post(create_short_url))
}