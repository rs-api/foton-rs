use rust_api::prelude::*;
use serde::Serialize;
use std::sync::Arc;
use std::sync::atomic::{AtomicU64, Ordering};

#[derive(Clone)]
struct AppState {
    counter: Arc<AtomicU64>,
    app_name: String,
}

#[derive(Serialize)]
struct CountResponse {
    count: u64,
    app_name: String,
}

#[tokio::main]
async fn main() {
    let state = AppState {
        counter: Arc::new(AtomicU64::new(0)),
        app_name: "State Demo".to_string(),
    };

    let app = RustApi::with_state(state)
        .get("/", |_req: Req| async {
            Res::text("Visit /count or /increment")
        })
        .get("/count", |State(state): State<AppState>| async move {
            let count = state.counter.load(Ordering::SeqCst);
            Res::json(&CountResponse {
                count,
                app_name: state.app_name.clone(),
            })
        })
        .post("/increment", |State(state): State<AppState>| async move {
            let count = state.counter.fetch_add(1, Ordering::SeqCst) + 1;
            Res::json(&CountResponse {
                count,
                app_name: state.app_name.clone(),
            })
        });

    app.listen(([127, 0, 0, 1], 3001)).await.unwrap();
}
