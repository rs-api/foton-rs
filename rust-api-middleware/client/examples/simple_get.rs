//! GET request example.

use rust_api_client::Client;
use tokio::io::{self, AsyncWriteExt as _};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let url = std::env::args()
        .nth(1)
        .unwrap_or_else(|| "https://httpbin.org/get".to_string());

    println!("Fetching: {}", url);

    let client = Client::new();
    let res = client.get(&url).await?;

    println!("Status: {}", res.status());
    println!("Headers: {:#?}\n", res.headers());

    let body = Client::body_text(res).await?;
    io::stdout().write_all(body.as_bytes()).await?;

    println!("\n\nDone!");
    Ok(())
}
