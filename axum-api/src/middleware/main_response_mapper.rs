use std::time::SystemTime;

use axum::response::Response;
use axum::response::Result;
use axum::{
    http::{Method, Uri},
    response::IntoResponse,
    Json,
};
use bson::Uuid;
use serde::Serialize;
use serde_json::{json, Value};
use serde_with::skip_serializing_none;

use crate::error::{ClientError, Error};

#[skip_serializing_none]
#[derive(Serialize)]
struct RequestLogLine {
    uuid: String,
    timestamp: String,

    // -- http request attributes
    req_path: String,
    req_method: String,

    // -- Errors attributes
    client_error_type: Option<String>,
    error_type: Option<String>,
    error_data: Option<Value>,
}

async fn log_request(
    uuid: String,
    req_method: Method,
    uri: Uri,
    service_error: Option<Error>,
    client_error: Option<ClientError>,
) -> Result<()> {
    let timestamp = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap()
        .as_millis();

    let error_type = service_error.clone().map(|se| se.as_ref().to_string());
    let error_data = serde_json::to_value(service_error)
        .ok()
        .and_then(|mut v| v.get_mut("data").map(|v| v.take()));

    // Create the RequestLogLine
    let log_line = RequestLogLine {
        uuid,
        timestamp: timestamp.to_string(),

        req_path: uri.path().to_string(),
        req_method: req_method.to_string(),

        client_error_type: client_error.map(|ce| ce.as_ref().to_string()),

        error_type,
        error_data,
    };

    println!(">> Log Line: \n{:?}", json!(log_line));

    Ok(())
}

pub async fn main_response_mapper(uri: Uri, req_method: Method, res: Response) -> Response {
    let uuid = Uuid::new();

    // Get the eventual response error
    let service_error = res.extensions().get::<Error>();
    let client_status_error = service_error.map(|e| e.client_status_and_error());

    // -- If client error, build the new response
    let error_response = client_status_error.as_ref().map(|(status, client_error)| {
        let client_error_body = json!({"error": {
                    "type": client_error.as_ref(),
                    "req_uuid": uuid.to_string(),
            }
        });

        println!(">> Client Error: {:?}", client_error_body);
        (*status, Json(client_error_body)).into_response()
    });

    println!(
        ">> Server Log line - {uuid} - Error: {error:?}",
        uuid = uuid,
        error = client_status_error
    );

    // Build and log the request log line
    let client_error = client_status_error.unzip().1;
    log_request(
        uuid.to_string(),
        req_method,
        uri,
        service_error.cloned(),
        client_error,
    )
    .await
    .unwrap();

    println!();
    error_response.unwrap_or(res)
}

//OLD
// async fn main_response_mapper(
//     ctx: Option<Ctx>,
//     uri: Uri,
//     req_method: Method,
//     res: Response,
// ) -> Response {
//     println!("->> {:<12} - main_response_mapper", "RES_MAPPER");
//     let uuid = Uuid::new_v4();

//     // -- Get the eventual response error.
//     let service_error = res.extensions().get::<Error>();
//     let client_status_error = service_error.map(|se| se.client_status_and_error());

//     // -- If client error, build the new reponse.
//     let error_response = client_status_error
//         .as_ref()
//         .map(|(status_code, client_error)| {
//             let client_error_body = json!({
//                 "error": {
//                     "type": client_error.as_ref(),
//                     "req_uuid": uuid.to_string(),
//                 }
//             });

//             println!("    ->> client_error_body: {client_error_body}");

//             // Build the new response from the client_error_body
//             (*status_code, Json(client_error_body)).into_response()
//         });

//     // Build and log the server log line.
//     let client_error = client_status_error.unzip().1;
//     // TODO: Need to hander if log_request fail (but should not fail request)
//     let _ = log_request(uuid, req_method, uri, ctx, service_error, client_error).await;

//     println!();
//     error_response.unwrap_or(res)
// }
