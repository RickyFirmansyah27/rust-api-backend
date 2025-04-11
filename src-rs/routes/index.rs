use vercel_runtime::{Body, Error, Request, Response};
use crate::controllers::{hello_controller, user_controller};
use crate::helpers::base_response;

pub async fn route_handler(req: Request) -> Result<Response<Body>, Error> {
    let method = req.method().as_str();
    let path = req.uri().path();
    
    match (method, path) {
        ("GET", "/") => base_response::success("Welcome to Rust Service", None),
        ("GET", "/hello") => hello_controller::handler(req).await,
        ("POST", "/user") => user_controller::handler(req).await,
        _ => base_response::success("Not Route found", None),
    }
}