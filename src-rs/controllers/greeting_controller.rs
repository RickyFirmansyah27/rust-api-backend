use serde_json::json;
use vercel_runtime::{Body, Error, Request, Response};
use crate::helpers::base_response;

pub async fn handler(_req: Request) -> Result<Response<Body>, Error> {
    let data = json!({ "greeting": "Hello from Rust Service" });
    base_response::success("Request successful", Some(data))
}