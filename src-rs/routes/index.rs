use serde_json::json;
use vercel_runtime::{Body, Error, Request, Response, StatusCode};
use crate::controllers::{hello_controller, user_controller};

pub async fn route_handler(req: Request) -> Result<Response<Body>, Error> {
    let method = req.method().as_str();
    let path = req.uri().path();
    
    match (method, path) {
        ("GET", "/") => {
            let body = json!({
                "status": true,
                "message": "Welcome to Rust Service",
            });
        
            Ok(Response::builder()
                .status(StatusCode::CREATED)
                .header("Content-Type", "application/json")
                .body(body.to_string().into())?)
        },        
        ("GET", "/hello") => hello_controller::handler(req).await,
        ("POST", "/user") => user_controller::handler(req).await,
        _ => {
            let body = json!({
                "status": false,
                "message": "No Route Found",
            });
        
            Ok(Response::builder()
                .status(StatusCode::NOT_FOUND)
                .header("Content-Type", "application/json")
                .body(body.to_string().into())?)
        },        
    }
}