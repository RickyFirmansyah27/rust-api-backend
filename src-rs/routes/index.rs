// src/routes/index.rs atau src-rs/routes/index.rs
use serde_json::json;
use vercel_runtime::{Body, Error, Request, Response, StatusCode};
use crate::controllers::{greeting_controller, hello_controller, user_controller};

pub async fn route_handler(req: Request) -> Result<Response<Body>, Error> {
    let method = req.method().as_str();
    let path = req.uri().path();
    
    match (method, path) {
        ("GET", "/") => greeting_controller::handler(req).await,
        ("GET", "/hello") => hello_controller::handler(req).await,
        ("POST", "/user") => user_controller::handler(req).await,
        _ => {
            // Route not found
            Ok(Response::builder()
                .status(StatusCode::NOT_FOUND)
                .header("Content-Type", "application/json")
                .body(
                    json!({
                        "error": "Route not found",
                    })
                    .to_string()
                    .into(),
                )?)
        }
    }
}