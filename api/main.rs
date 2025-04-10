use vercel_runtime::{run, Body, Error, Request, Response, StatusCode};
mod routes;

#[tokio::main]
async fn main() -> Result<(), Error> {
    run(handler).await
}

pub async fn handler(req: Request) -> Result<Response<Body>, Error> {
    let path = req.uri().path();

    match path {
        "/hello" => routes::hello::handle(req).await,
        "/goodbye" => routes::goodbye::handle(req).await,
        _ => Ok(Response::builder()
            .status(StatusCode::NOT_FOUND)
            .body("Not Found".into())?),
    }
}
