// Not tested

use rust_api::prelude::*;
use rust_api_cors::Cors;
use std::sync::Arc;

#[tokio::main]
async fn main() {
    let cors = Cors::permissive();

    let app = RustApi::new()
        .layer(from_fn(move |req: Req, state: Arc<()>, next: Next| {
            let cors = cors.clone();
            async move { cors.handle(req, state, next).await }
        }))
        .get("/", |_req: Req| async { Res::text("Hello with CORS!") })
        .get("/api/users", |_req: Req| async {
            Res::json(&serde_json::json!({
                "users": ["Alice", "Bob", "Charlie"]
            }))
        })
        .post("/api/users", |_req: Req| async {
            Res::json(&serde_json::json!({
                "success": true,
                "message": "User created"
            }))
        });

    app.listen(([127, 0, 0, 1], 3040)).await.unwrap();
}
