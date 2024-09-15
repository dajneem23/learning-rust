use axum::{
    routing::{get, get_service},
    Router,
};
use tower_http::services::ServeDir;

pub fn static_routes() -> Router {
    Router::new().nest_service("/", get_service(ServeDir::new("./")))
}
