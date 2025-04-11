use serde_json::{json, Value};
use vercel_runtime::{Body, Response, StatusCode, Error};

pub fn success(message: &str, data: Option<Value>) -> Result<Response<Body>, Error> {
    let mut body = json!({
        "status": true,
        "message": message,
        "statusCode": 200,
    });

    if let Some(val) = data {
        let is_empty_array = val.is_array() && val.as_array().unwrap().is_empty();
        if !is_empty_array {
            body["data"] = val;
        }
    }

    Ok(Response::builder()
        .status(StatusCode::OK)
        .header("Content-Type", "application/json")
        .body(body.to_string().into())?)
}


pub fn created(message: &str) -> Result<Response<Body>, Error> {
    let body = json!({
        "status": true,
        "message": message,
    });

    Ok(Response::builder()
        .status(StatusCode::CREATED)
        .header("Content-Type", "application/json")
        .body(body.to_string().into())?)
}

pub fn error(message: &str, err: Option<Value>) -> Result<Response<Body>, Error> {
    let body = json!({
        "status": false,
        "message": format!("{}{}", message, match err {
            Some(e) => format!(": {}", e),
            None => "".to_string(),
        }),
        "statusCode": 400
    });

    Ok(Response::builder()
        .status(StatusCode::BAD_REQUEST)
        .header("Content-Type", "application/json")
        .body(body.to_string().into())?)
}

pub fn unauthorized(message: &str) -> Result<Response<Body>, Error> {
    let body = json!({
        "status": false,
        "message": message
    });

    Ok(Response::builder()
        .status(StatusCode::FORBIDDEN)
        .header("Content-Type", "application/json")
        .body(body.to_string().into())?)
}

pub fn internal_server_error(message: &str, err: Option<Value>) -> Result<Response<Body>, Error> {
    let body = json!({
        "status": false,
        "message": format!("{}{}", message, match err {
            Some(e) => format!(": {}", e),
            None => "".to_string(),
        })
    });

    Ok(Response::builder()
        .status(StatusCode::INTERNAL_SERVER_ERROR)
        .header("Content-Type", "application/json")
        .body(body.to_string().into())?)
}
