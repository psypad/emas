use axum::{
    Json, Router,
    extract::State,
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
};
use serde::Deserialize;
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
        .layer(cors);

    println!("server listening on: http://localhost:3000/");

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}