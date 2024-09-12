#[derive(Debug, serde::Deserialize)]
struct HelloQuery {
    name: Option<String>,
}

use axum::{
    extract::Query,
    response::{Html, IntoResponse},
    routing::get,
    Router,
};
#[allow(unused)]
use tokio::net::TcpListener;

pub fn hello_routes() -> Router {
    Router::new().route("/hello", get(hello_handler))
}

async fn hello_handler(params: Query<HelloQuery>) -> impl IntoResponse {
    let binding = "World".to_string();
    let name = params.name.as_ref().unwrap_or(&binding);
    println!("Hello, {}!", name);
    Html(format!("<h1>Hello {name} </h1>"))
}
