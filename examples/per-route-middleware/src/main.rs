use rust_api::prelude::*;

#[tokio::main]
async fn main() {
    let app = RustApi::new()
        .layer(from_fn(|req, _state, next| async move {
            println!("[Global] {} {}", req.method(), req.path());
            next.run(req).await
        }))
        .get("/", |_req: Req| async { Res::text("Public route") })
        .route(
            Route::get("/admin", |_req: Req| async { Res::text("Admin panel") }).layer(from_fn(
                |req, _state, next| async move {
                    if let Some(auth) = req.header("authorization") {
                        if auth.starts_with("Bearer ") {
                            return next.run(req).await;
                        }
                    }
                    Res::builder().status(401).text("Unauthorized")
                },
            )),
        )
        .route(
            Route::post("/api/data", |_req: Req| async { Res::text("Data created") }).layer(
                from_fn(|req, _state, next| async move {
                    if let Some(ct) = req.header("content-type") {
                        if ct.contains("application/json") {
                            return next.run(req).await;
                        }
                    }
                    Res::builder()
                        .status(400)
                        .text("Content-Type must be application/json")
                }),
            ),
        );

    app.listen(([127, 0, 0, 1], 3008)).await.unwrap();
}
