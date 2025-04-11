// src-rs/routes/user.rs
use serde_json::json;
use vercel_runtime::{Body, Error, Request, Response, StatusCode};

pub async fn handler(_req: Request) -> Result<Response<Body>, Error> {
    // In a real scenario, you'd parse the request body
    // For this example, we're just returning a simple object
    Ok(Response::builder()
        .status(StatusCode::OK)
        .header("Content-Type", "application/json")
        .body(
            json!({
                "username": "John Doe",
            })
            .to_string()
            .into(),
        )?)
}