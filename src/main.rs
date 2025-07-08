mod operations;
mod variables;

use axum::{routing::get, Router};
use dotenvy::dotenv;

#[tokio::main]
async fn main() {
    dotenv().ok();
    let address = "0.0.0.0:6570";
    let listener = tokio::net::TcpListener::bind(address).await.unwrap();
    let router = Router::new().route("/", get(operations::reagents_get));
    axum::serve(listener, router).await.unwrap();
}
