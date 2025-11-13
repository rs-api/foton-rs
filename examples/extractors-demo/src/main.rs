use rust_api::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
struct UserPath {
    id: String,
}

#[derive(Deserialize)]
struct SearchQuery {
    q: String,
    page: Option<u32>,
}

#[derive(Deserialize, Serialize)]
struct CreateUser {
    name: String,
    email: String,
}

#[derive(Deserialize)]
struct LoginForm {
    username: String,
    password: String,
}

#[tokio::main]
async fn main() {
    let app = RustApi::new()
        .get("/users/{id}", |Path(params): Path<UserPath>| async move {
            Res::text(format!("User ID: {}", params.id))
        })
        .get("/search", |Query(query): Query<SearchQuery>| async move {
            Res::text(format!(
                "Search: {} (page {})",
                query.q,
                query.page.unwrap_or(1)
            ))
        })
        .post("/users", |Json(user): Json<CreateUser>| async move {
            Res::json(&serde_json::json!({ "success": true, "user": user }))
        })
        .post("/login", |Form(form): Form<LoginForm>| async move {
            if form.username == "admin" && form.password == "secret" {
                Res::text("Login successful")
            } else {
                Res::builder().status(401).text("Invalid credentials")
            }
        })
        .get("/headers", |Headers(headers): Headers| async move {
            let auth = headers
                .get("authorization")
                .and_then(|v| v.to_str().ok())
                .unwrap_or("none");
            Res::text(format!("Auth: {}", auth))
        })
        .post("/upload", |BodyBytes(data): BodyBytes| async move {
            Res::text(format!("Uploaded {} bytes", data.len()))
        })
        .post(
            "/posts/{id}/comments",
            |Path(path): Path<UserPath>, Json(body): Json<CreateUser>| async move {
                Res::json(&serde_json::json!({
                    "post_id": path.id,
                    "comment": body
                }))
            },
        );

    app.listen(([127, 0, 0, 1], 3030)).await.unwrap();
}
