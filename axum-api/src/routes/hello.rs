#[derive(Debug, serde::Deserialize)]
struct HelloQuery {
    name: Option<String>,
}

use axum::{
    extract::{Path, Query},
    response::{Html, IntoResponse},
    routing::get,
    Router,
};
#[allow(unused)]
use tokio::net::TcpListener;

pub fn hello_routes() -> Router {
    Router::new()
        .route("/hello", get(hello_handler))
        .route("/hello2/:name", get(hello2_handler))
}

async fn hello_handler(params: Query<HelloQuery>) -> impl IntoResponse {
    let binding = "World".to_string();
    let name = params.name.as_ref().unwrap_or(&binding);
    println!("handler=>>Hello, {}!", name);
    Html(format!("<h1>Hello {name} </h1>"))
}
async fn hello2_handler(Path((name,)): Path<(String,)>) -> impl IntoResponse {
    println!("handler=>>Hello, {}!", name);
    Html(format!("<h1>Hello {name} </h1>"))
}
