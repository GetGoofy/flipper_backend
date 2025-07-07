mod operations;

use axum::{routing::get, Router};

#[tokio::main]
async fn main() {
    let address = "0.0.0.0:6570";
    let listener = tokio::net::TcpListener::bind(address).await.unwrap();
    let router = Router::new().route("/", get(operations::reagents_get));

    axum::serve(listener, router).await.unwrap();
}
