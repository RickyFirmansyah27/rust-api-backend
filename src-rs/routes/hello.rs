use vercel_runtime::{Body, Error, Request, Response, StatusCode};
use serde_json::json;

pub async fn handle(_req: Request) -> Result<Response<Body>, Error> {
    let body = json!({ "message": "Hello from Rust!" }).to_string();

    Ok(Response::builder()
        .status(StatusCode::OK)
        .header("Content-Type", "application/json")
        .body(body.into())?)
}
