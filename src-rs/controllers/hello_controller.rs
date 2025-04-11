use vercel_runtime::{Body, Error, Request, Response};
use crate::helpers::base_response;
use crate::helpers::logger;

pub async fn handler(_req: Request) -> Result<Response<Body>, Error> {
    logger::info_msg("[HelloController] - Info");
    let message = "Hello from Rust Service";
    base_response::success(message, None)
}