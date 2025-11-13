use rust_api::prelude::*;
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Clone, Debug)]
struct User {
    id: u64,
    username: String,
}

#[derive(Clone, Debug)]
struct RequestId(String);

fn generate_id() -> String {
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_micros();
    format!("{:x}", timestamp)
}

#[tokio::main]
async fn main() {
    let app = RustApi::new()
        .layer(from_fn(|mut req: Req, _state, next| async move {
            let id = RequestId(generate_id());
            req.extensions_mut().insert(id);
            next.run(req).await
        }))
        .layer(from_fn(|mut req: Req, _state, next| async move {
            if let Some(auth) = req.header("authorization") {
                if auth.starts_with("Bearer ") {
                    let user = User {
                        id: 42,
                        username: "alice".to_string(),
                    };
                    req.extensions_mut().insert(user);
                }
            }
            next.run(req).await
        }))
        .get("/", |req: Req| async move {
            let id = req.extensions().get::<RequestId>();
            let user = req.extensions().get::<User>();

            match user {
                Some(u) => Res::text(format!("Hello, {}! (ID: {:?})", u.username, id)),
                None => Res::text(format!("Hello, guest! (ID: {:?})", id)),
            }
        })
        .get("/admin", |req: Req| async move {
            match req.extensions().get::<User>() {
                Some(user) => Res::text(format!("Admin: {}", user.username)),
                None => Res::builder().status(401).text("Unauthorized"),
            }
        });

    app.listen(([127, 0, 0, 1], 3007)).await.unwrap();
}
