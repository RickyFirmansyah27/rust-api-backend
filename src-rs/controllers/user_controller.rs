// src-rs/controllers/user_controller.rs
use serde_json::json;
use vercel_runtime::{Body, Error, Request, Response, StatusCode};

pub async fn handler(_req: Request) -> Result<Response<Body>, Error> {
    Ok(Response::builder()
        .status(StatusCode::CREATED)
        .header("Content-Type", "application/json")
        .body(
            json!({
                "message": "User created successfully",
                "status": "success"
            })
            .to_string()
            .into(),
        )?)
}
