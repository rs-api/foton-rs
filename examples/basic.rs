use rust_api::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone)]
struct AppState {
    message: String,
}

#[derive(Serialize)]
struct HelloResponse {
    message: String,
}

#[derive(Serialize, Deserialize)]
struct User {
    name: String,
    email: String,
}

async fn home(_req: Req) -> Res {
    Res::text("Welcome to Rust Api!")
}

async fn hello(_req: Req) -> Res {
    Res::json(&HelloResponse {
        message: "Hello, world!".to_string(),
    })
}

async fn health(_req: Req) -> Res {
    Res::builder().status(200).json(&serde_json::json!({
        "status": "healthy",
        "version": "0.0.1"
    }))
}

#[tokio::main]
async fn main() {
    let state = AppState {
        message: "Hello from state!".to_string(),
    };

    // API v1 routes
    let api_v1 = Router::new().get("/hello", hello);

    let app = RustApi::with_state(state)
        .get("/", home)
        .get("/health", health)
        .nest("/api/v1", api_v1);

    println!("Starting server...");
    app.listen(([127, 0, 0, 1], 3000))
        .await
        .expect("Failed to start server");
}
