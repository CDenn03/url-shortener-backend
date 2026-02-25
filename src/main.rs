use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::{IntoResponse, Redirect, Response},
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use tower_http::cors::{Any, CorsLayer};
use tracing::{error, info};

/* ============================
   App State
   ============================ */

#[derive(Clone)]
struct AppState {
    db: PgPool,
}

/* ============================
   Request / Response Models
   ============================ */

#[derive(Deserialize)]
struct CreateLinkRequest {
    url: String,
    custom_code: Option<String>,
}

#[derive(Serialize)]
struct CreateLinkResponse {
    short_code: String,
    short_url: String,
}

/* ============================
   API Response Envelope
   ============================ */

#[derive(Serialize)]
struct ApiSuccess<T> {
    success: bool,
    data: T,
}

#[derive(Serialize)]
struct ApiErrorBody {
    success: bool,
    error: ApiErrorDetail,
}

#[derive(Serialize)]
struct ApiErrorDetail {
    code: &'static str,
    message: String,
}

/* ============================
   Centralized Error Handling
   ============================ */

#[derive(thiserror::Error, Debug)]
pub enum AppError {
    #[error("invalid input: {0}")]
    Validation(String),

    #[error("not found")]
    NotFound,

    #[error("conflict")]
    Conflict,

    #[error("gone")]
    Gone,

    #[error("database error")]
    Database(#[from] sqlx::Error),

    #[error("internal error")]
    Internal,
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, code, message) = match &self {
            AppError::Validation(msg) => (
                StatusCode::BAD_REQUEST,
                "VALIDATION_ERROR",
                msg.clone(),
            ),

            AppError::NotFound => (
                StatusCode::NOT_FOUND,
                "NOT_FOUND",
                "not found".into(),
            ),

            AppError::Conflict => (
                StatusCode::CONFLICT,
                "CONFLICT",
                "already exists".into(),
            ),

            AppError::Gone => (
                StatusCode::GONE,
                "GONE",
                "link expired".into(),
            ),

            AppError::Database(e) => {
                error!("database error: {:?}", e);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "INTERNAL_ERROR",
                    "internal error".into(),
                )
            }

            AppError::Internal => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "INTERNAL_ERROR",
                "internal error".into(),
            ),
        };

        let body = ApiErrorBody {
            success: false,
            error: ApiErrorDetail { code, message },
        };

        (status, Json(body)).into_response()
    }
}

/* ============================
   Application Entry
   ============================ */

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::dotenv().ok();
    tracing_subscriber::fmt::init();

    let database_url =
        std::env::var("DATABASE_URL").expect("DATABASE_URL required");

    let pool = PgPool::connect(&database_url).await?;

    let state = AppState { db: pool };

    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    let app = Router::new()
        .route("/api/shorten", post(create_short_link))
        .route("/:code", get(redirect_handler))
        .route("/health", get(health_check))
        .layer(cors)
        .with_state(state);

    let listener =
        tokio::net::TcpListener::bind("0.0.0.0:8080").await?;

    info!("Server running on http://0.0.0.0:8080");

    axum::serve(listener, app).await?;

    Ok(())
}

/* ============================
   Handlers
   ============================ */

async fn create_short_link(
    State(state): State<AppState>,
    Json(payload): Json<CreateLinkRequest>,
) -> Result<impl IntoResponse, AppError> {

    // Basic validation
    if payload.url.len() < 8 || !payload.url.starts_with("http") {
        return Err(AppError::Validation("invalid URL".into()));
    }

    let code = payload
        .custom_code
        .unwrap_or_else(|| nanoid::nanoid!(8));

    let insert = sqlx::query!(
        r#"
        INSERT INTO links (short_code, original_url)
        VALUES ($1, $2)
        RETURNING id
        "#,
        &code,
        payload.url.trim()
    )
        .fetch_one(&state.db)
        .await;

    match insert {
        Ok(_) => {
            let short_url = format_short_url(&code);

            Ok((
                StatusCode::CREATED,
                Json(ApiSuccess {
                    success: true,
                    data: CreateLinkResponse {
                        short_code: code,
                        short_url,
                    },
                }),
            ))
        }

        Err(sqlx::Error::Database(db_err))
        if db_err
            .constraint()
            .is_some_and(|c| c.contains("short_code") || c.contains("unique"))
            || db_err.message().contains("unique constraint") =>
            {
                Err(AppError::Conflict)
            }

        Err(e) => Err(AppError::Database(e)),
    }
}

async fn redirect_handler(
    State(state): State<AppState>,
    Path(code): Path<String>,
) -> Result<impl IntoResponse, AppError> {

    let link = sqlx::query!(
        r#"
        SELECT id, original_url, expires_at
        FROM links
        WHERE short_code = $1
          AND is_active = true
        "#,
        code
    )
        .fetch_optional(&state.db)
        .await?; // auto converts to AppError::Database

    let Some(link) = link else {
        return Err(AppError::NotFound);
    };

    if let Some(expires) = link.expires_at {
        if expires < chrono::Utc::now() {
            return Err(AppError::Gone);
        }
    }

    // Fire-and-forget click tracking
    let db = state.db.clone();
    let link_id = link.id;

    tokio::spawn(async move {
        if let Err(e) = sqlx::query!(
            "INSERT INTO clicks (link_id) VALUES ($1)",
            link_id
        )
            .execute(&db)
            .await
        {
            error!("Failed to log click: {:?}", e);
        }
    });

    Ok(Redirect::temporary(&link.original_url))
}

async fn health_check() -> impl IntoResponse {
    Json(ApiSuccess {
        success: true,
        data: "OK",
    })
}

/* ============================
   Helpers
   ============================ */

fn format_short_url(code: &str) -> String {
    format!("http://localhost:8080/{code}")
}