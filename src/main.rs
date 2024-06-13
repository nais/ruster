use axum::{Json, Router};
use axum::routing::post;
use serde::{Deserialize, Serialize};

mod old;
mod mymodule;
mod fib;

#[tokio::main]
async fn main() {
    let router = Router::new().route("/", post(handler));
    let listener = tokio::net::TcpListener::bind("127.0.0.1:6000").await.unwrap();
    axum::serve(listener, router).await.unwrap();
}

#[derive(Deserialize)]
struct HelloRequest {
    pub msg: String
}

#[derive(Serialize)]
struct HelloResponse {
    pub msg: String
}

async fn handler(Json(payload): Json<HelloRequest>) -> Json<HelloResponse> {
    Json(HelloResponse{msg: format!("you said: {}", payload.msg).into()})
}