use rust_api::prelude::*;
use std::sync::Arc;
use std::sync::atomic::{AtomicU32, Ordering};

#[derive(Clone)]
struct AppState {
    counter: Arc<AtomicU32>,
}

#[tokio::main]
async fn main() {
    let state = AppState {
        counter: Arc::new(AtomicU32::new(0)),
    };

    let app = RustApi::with_state(state)
        .layer(from_fn(|req: Req, state: Arc<AppState>, next| async move {
            let count = state.counter.fetch_add(1, Ordering::SeqCst) + 1;
            println!("Middleware 1: Request #{}", count);
            next.run(req).await
        }))
        .layer(from_fn(|req: Req, state: Arc<AppState>, next| async move {
            let count = state.counter.load(Ordering::SeqCst);
            println!("Middleware 2: Counter at {}", count);
            next.run(req).await
        }))
        .get("/", |State(state): State<AppState>| async move {
            let count = state.counter.load(Ordering::SeqCst);
            Res::text(format!("Request #{}", count))
        });

    app.listen(([127, 0, 0, 1], 3005)).await.unwrap();
}
