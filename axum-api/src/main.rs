mod routes;
use axum::{http::StatusCode, response::Html, routing::get, Json, Router};
use routes::hello::hello_routes;
#[allow(unused)]
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    let routes_all = Router::new().merge(hello_routes());
    // .route("/", axum::handler::get(handler))
    // .route("/users", axum::handler::post(create_user))
    // .route("/users/:id", axum::handler::get(get_user))
    // .route("/users/:id", axum::handler::delete(delete_user));

    //region: -- start server --

    let listener = TcpListener::bind("127.0.0.1:8080").await.unwrap();
    println!("->> LISTENING on {:?}\n", listener.local_addr());
    axum::serve(listener, routes_all.into_make_service())
        .await
        .unwrap();
    //endregion: -- start server --
}
