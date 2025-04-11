use vercel_runtime::{Body, Error, Request, Response};
use crate::helpers::base_response;

pub async fn handler(_req: Request) -> Result<Response<Body>, Error> {
    let message = "Hello from Rust Service";
    base_response::success(message, None)
}