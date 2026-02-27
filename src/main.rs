use std::env;
use axum::{Router, routing::get, extract::State, Json};
use sqlx::PgPool;
use serde_json::json;

mod db;

async fn health_check(State(pool): State<PgPool>) -> Json<serde_json::Value> {
    match sqlx::query("SELECT 1").execute(&pool).await {
        Ok(_) => Json(json!({
            "status": "healthy",
            "database": "connected"
        })),
        Err(_) => Json(json!({
            "status": "unhealthy",
            "database": "disconnected"
        }))
    }
}

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");

    let pool = db::connect_db(&database_url).await;

    let app = Router::new()
        .route("/health", get(health_check))
        .with_state(pool);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:5000")
        .await
        .expect("Failed to bind to port 5000");

    println!("🚀 Server running on http://0.0.0.0:5000");

    axum::serve(listener, app).await.unwrap();
}