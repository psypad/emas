use axum::{
    Json, Router,
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
};
use serde::Deserialize;
use serde_json::json;
use std::net::SocketAddr;
use tower_http::cors::{CorsLayer, Any};

#[derive(Deserialize)]
struct LoginRequest {
    email: String,
    password: String,
}

#[derive(Deserialize)]
struct RegisterRequest{
    name: String,
    email: String,
    password: String,
}

async fn login_handler(Json(payload): Json<LoginRequest>) -> impl IntoResponse {
    println!("Received login: {} / {}", payload.email, payload.password);
    
    let dummy_token = "abc123";

    let body = serde_json::json!({
        "token": dummy_token,
        "message": "Login success"
    });

    (StatusCode::OK, Json(body))
}

async fn register_handler(Json(payload): Json<RegisterRequest>) -> impl IntoResponse {
    println!("Received Registration: {} / {} / {}", payload.name  , payload.email, payload.password);


    let body = serde_json::json!({
        "message": "Register success"
    });

    (StatusCode::OK, Json(body))
}

#[tokio::main]
async fn main() {

    let cors = CorsLayer::new()
    .allow_origin(Any)
    .allow_methods(Any)
    .allow_headers(Any);


    // build our application with a single route
    let app = Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .route("/api/auth/login", post(login_handler))
        .route("/api/auth/register", post(register_handler))
        .route("/api/keys", get(get_keys).post(generate_key))
        .route("/api/metrics", get(get_metrics))
        .route("/api/logs", get(get_logs))
        .route("/health", get(health_check_handler))
        .layer(cors);

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("Running on http://{}", addr);

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn health_check_handler() -> impl IntoResponse {
    let healthy = true; // Later, you can insert DB or Redis checks here

    if healthy {
        let response = json!({
            "status": "ok",
            "uptime": "running",
            "message": "API backend is healthy",
            "code": 200
        });

        (StatusCode::OK, Json(response))
    } else {
        let response = json!({
            "status": "error",
            "message": "Service unavailable"
        });

        (StatusCode::SERVICE_UNAVAILABLE, Json(response))
    }
}


async fn get_keys() -> Json<serde_json::Value> {
    Json(json!({ "keys": ["key-abc123", "key-xyz789"] }))
}

async fn generate_key(Json(payload): Json<serde_json::Value>) -> Json<serde_json::Value> {
    println!("Generating key for {:?}", payload);
    Json(json!({ "key": "key-generated" }))
}

async fn get_metrics() -> Json<serde_json::Value> {
    Json(json!({ "status": "up" }))
}

async fn get_logs() -> Json<serde_json::Value>  {
    Json(json!({ 
        "GET /api/users ": "200 OK", 
        "POST /api/data ": "401 Unauthorized",
    }
    ))
}