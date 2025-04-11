use vercel_runtime::{run, Error};

use myapp::routes::index::route_handler;
use myapp::helpers::logger;

#[tokio::main]
async fn main() -> Result<(), Error> {
    logger::init();
    run(route_handler).await
}