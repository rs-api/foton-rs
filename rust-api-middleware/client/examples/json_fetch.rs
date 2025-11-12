//! JSON fetch example.

use rust_api_client::Client;
use serde::{Deserialize, Serialize};
use std::time::Duration;

#[derive(Debug, Serialize, Deserialize)]
struct User {
    id: u64,
    name: String,
    email: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let client = Client::new().with_timeout(Duration::from_secs(10));

    // Fetch JSON data
    let url = "https://jsonplaceholder.typicode.com/users/1";
    println!("Fetching: {}", url);

    let res = client.get(url).await?;
    println!("Status: {}", res.status());

    let user: User = Client::body_json(res).await?;
    println!("User: {:#?}\n", user);

    // Or with the shorthand method
    let user2: User = client
        .get_json("https://jsonplaceholder.typicode.com/users/2")
        .await?;
    println!("User (shorthand): {:#?}\n", user2);

    // POST JSON data
    let new_user = User {
        id: 999,
        name: "Test User".to_string(),
        email: "test@example.com".to_string(),
    };

    println!("Posting JSON...");
    let res = client
        .post_json("https://jsonplaceholder.typicode.com/users", &new_user)
        .await?;
    println!("Status: {}", res.status());

    let created: User = Client::body_json(res).await?;
    println!("Created: {:#?}", created);

    Ok(())
}
