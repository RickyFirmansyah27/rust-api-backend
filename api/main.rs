use serde_json::json;
use vercel_runtime::{run, Body, Error, Request, Response, StatusCode};
use myapp::routes::{hello, index, user};

#[tokio::main]
async fn main() -> Result<(), Error> {
    run(handler).await
}

pub async fn handler(req: Request) -> Result<Response<Body>, Error> {
    // Get path and method
    let path = req.uri().path();
    let method = req.method().as_str();
    
    match (method, path) {
        ("GET", "/") => index::handler(req).await,
        ("GET", "/hello") => hello::handler(req).await,
        ("POST", "/user") => user::handler(req).await,
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