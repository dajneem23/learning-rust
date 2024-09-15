use crate::{error::ClientError, web, Error, Result};
use axum::{body, Json, Router};
use serde::Deserialize;
use serde_json::Value;
use tower_cookies::{Cookie, Cookies};
#[derive(Debug, Clone, Deserialize)]
struct LoginPayload {
    username: String,
    password: String,
}

pub fn login_routes() -> Router {
    Router::new().route("/api/login", axum::routing::post(api_login))
}

async fn api_login(cookies: Cookies, payload: Json<LoginPayload>) -> Result<Json<(Value)>> {
    println!("api_login=>> {:?}", payload);
    if payload.username == "admin" && payload.password == "admin" {
        //implement a token generation here
        cookies.add(Cookie::new(web::AUTH_TOKEN, "admin"));
        Ok(Json(
            serde_json::json!({ "status": "ok" ,  "result": "login ok"}),
        ))
    } else {
        Err(Error::WrongCredentials {
            message: "Wrong credentials".to_string(),
        })
    }

    //TODO: set a cookie with a token.
}
