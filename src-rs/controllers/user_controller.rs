use vercel_runtime::{Body, Error, Request, Response};
use crate::helpers::base_response;

pub async fn handler(_req: Request) -> Result<Response<Body>, Error> {
    base_response::success("User created successfully", None)
}
