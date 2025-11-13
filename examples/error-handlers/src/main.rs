use rust_api::prelude::*;

#[tokio::main]
async fn main() {
    let app = RustApi::new()
        .get("/", |_req: Req| async {
            Ok(Res::text("Try /error endpoints"))
        })
        .get("/error/400", |_req: Req| async {
            Err::<Res, _>(Error::bad_request("Invalid request"))
        })
        .get("/error/401", |_req: Req| async {
            Err::<Res, _>(Error::unauthorized("Not authorized"))
        })
        .get("/error/404", |_req: Req| async {
            Err::<Res, _>(Error::not_found("Resource not found"))
        })
        .get("/error/500", |_req: Req| async {
            Err::<Res, _>(Error::internal("Internal server error"))
        });

    app.listen(([127, 0, 0, 1], 3009)).await.unwrap();
}
