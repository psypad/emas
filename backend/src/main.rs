mod handle;

use axum::{
    Json, Router, http::StatusCode, response::{IntoResponse, Response}, routing::{get, post}, 
};
use tower_http::cors::{CorsLayer, Any};
use std::net::SocketAddr;
use serde_json::json;
use std::sync::Arc;
use sqlx::{postgres::PgPoolOptions, Pool, Postgres};
use validator::Validate;
use bcrypt::{hash, verify, DEFAULT_COST};
use jsonwebtoken::{encode, Header, EncodingKey};
use serde::{Deserialize, Serialize};
use chrono::Utc;
use handle::{LoginUserSchema, RegisterUserSchema};

#[derive(Debug)]
pub enum ApiError {
    ValidationError(String),
    DatabaseError(sqlx::Error),
    BcryptError(bcrypt::BcryptError),
    Unauthorized,
    Conflict(String),
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let (status, message) = match self {
            ApiError::ValidationError(msg) => (StatusCode::BAD_REQUEST, msg),
            ApiError::DatabaseError(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Database error".to_string()),
            ApiError::BcryptError(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Password hashing error".to_string()),
            ApiError::Unauthorized => (StatusCode::UNAUTHORIZED, "Invalid credentials".to_string()),
            ApiError::Conflict(msg) => (StatusCode::CONFLICT, msg),
        };
        let body = json!({ "error": message });
        (status, Json(body)).into_response()
    }
}

#[derive(Serialize, Deserialize)]
struct Claims {
    sub: String,
    exp: usize,
}

pub struct AppState {
    db: Pool<Postgres>,
}

async fn health_check_handler() -> impl IntoResponse {
    let response = json!({
        "status": "ok",
        "uptime": "running",
        "message": "API backend is healthy",
        "code": 200
    });
    (StatusCode::OK, Json(response))
}

async fn login_handler(
    axum::extract::State(state): axum::extract::State<Arc<AppState>>,
    Json(payload): Json<LoginUserSchema>,
) -> Result<impl IntoResponse, ApiError> {
    // Validate input
    payload.validate().map_err(|e| ApiError::ValidationError(e.to_string()))?;

    // Fetch user from database
    let user = sqlx::query!(
        "SELECT email, password_hash FROM users WHERE email = $1",
        payload.email
    )
    .fetch_optional(&state.db)
    .await
    .map_err(ApiError::DatabaseError)?
    .ok_or(ApiError::Unauthorized)?;

    // Verify password
    verify(&payload.password, &user.password_hash)
        .map_err(ApiError::BcryptError)?
        .then_some(())
        .ok_or(ApiError::Unauthorized)?;

    // Generate JWT
    let claims = Claims {
        sub: payload.email,
        exp: (Utc::now() + chrono::Duration::days(1)).timestamp() as usize,
    };
    let token = encode(&Header::default(), &claims, &EncodingKey::from_secret("secret".as_ref()))
        .map_err(|_| ApiError::ValidationError("Token generation failed".to_string()))?;

    let body = json!({ "token": token, "message": "Login success" });
    Ok((StatusCode::OK, Json(body)))
}

async fn register_handler(
    axum::extract::State(state): axum::extract::State<Arc<AppState>>,
    Json(payload): Json<RegisterUserSchema>,
) -> Result<impl IntoResponse, ApiError> {
    // Validate input
    payload.validate().map_err(|e| ApiError::ValidationError(e.to_string()))?;

    // Check if email already exists
    let exists = sqlx::query!("SELECT 1 AS exists FROM users WHERE email = $1", payload.email)
        .fetch_optional(&state.db)
        .await
        .map_err(ApiError::DatabaseError)?
        .is_some();
    if exists {
        return Err(ApiError::Conflict("Email already exists".to_string()));
    }

    // Hash password
    let hashed_password = hash(&payload.password, DEFAULT_COST)
        .map_err(ApiError::BcryptError)?;

    // Insert user into database
    sqlx::query!(
        "INSERT INTO users (name, email, password_hash) VALUES ($1, $2, $3)",
        payload.name, payload.email, hashed_password
    )
    .execute(&state.db)
    .await
    .map_err(ApiError::DatabaseError)?;

    let body = json!({ "message": "Register success" });
    Ok((StatusCode::OK, Json(body)))
}

async fn get_keys() -> Json<serde_json::Value> {
    Json(json!({ "keys": ["key-abc123", "key-xyz789"] }))
}

async fn generate_key(Json(payload): Json<serde_json::Value>) -> Json<serde_json::Value> {
    Json(json!({ "key": "key-generated" }))
}

async fn get_metrics() -> Json<serde_json::Value> {
    Json(json!({ "status": "up" }))
}

async fn get_logs() -> Json<serde_json::Value> {
    Json(json!({
        "GET /api/users": "200 OK",
        "POST /api/data": "401 Unauthorized",
    }))
}

#[tokio::main]
async fn main() {
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    dotenvy::dotenv().ok();
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let pool = PgPoolOptions::new()
        .max_connections(10)
        .connect(&database_url)
        .await
        .expect("Failed to connect to the database");

    let app_state = Arc::new(AppState { db: pool });

    let app = Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .route("/api/auth/login", post(login_handler))
        .route("/api/auth/register", post(register_handler))
        .route("/api/keys", get(get_keys).post(generate_key))
        .route("/api/metrics", get(get_metrics))
        .route("/api/logs", get(get_logs))
        .with_state(app_state)
        .layer(cors);

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("Running on http://{}", addr);

    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}