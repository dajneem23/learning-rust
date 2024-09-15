#[allow(unused)]
pub use self::error::{Error, Result};
use crate::ctx::Ctx;

mod ctx;
mod error;
mod middleware;
mod routes;
mod web;

use axum::{
    body::Body,
    http::{uri, Method, StatusCode, Uri},
    middleware::{from_fn, from_fn_with_state, map_response},
    response::{IntoResponse, Response},
    Router,
};
use tokio::net::TcpListener;
use tower::ServiceBuilder;
use tower_cookies::CookieManagerLayer;
use tower_http::{map_response_body::MapResponseBodyLayer, trace::TraceLayer};

use middleware::main_response_mapper::main_response_mapper;
use routes::{hello::hello_routes, login::login_routes, static_route::static_routes};

#[tokio::main]
async fn main() -> Result<()> {
    let routes_all = Router::new()
        .merge(hello_routes())
        .merge(login_routes())
        //should be after all routes
        .layer(TraceLayer::new_for_http())
        .layer(map_response(main_response_mapper))
        .layer(CookieManagerLayer::new())
        // .route("/", axum::handler::get(handler))
        .fallback_service(static_routes());
    // .route("/users", axum::handler::post(create_user))
    // .route("/users/:id", axum::handler::get(get_user))
    // .route("/users/:id", axum::handler::delete(delete_user));

    /* -- start server -- */
    let listener = TcpListener::bind("127.0.0.1:8080").await.unwrap();
    println!("->> LISTENING on {:?}\n", listener.local_addr());
    axum::serve(listener, routes_all.into_make_service())
        .await
        .unwrap();
    /* -- start server -- */

    Ok(())
}
