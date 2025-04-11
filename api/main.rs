use vercel_runtime::{run, Error};

use myapp::routes::index::route_handler;

#[tokio::main]
async fn main() -> Result<(), Error> {
    run(route_handler).await
}