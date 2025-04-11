use log::info;
use serde::Serialize;

pub fn init() {
    env_logger::init();
}

pub fn info_msg(message: &str) {
    info!("{}", message);
}

pub fn info<T>(message: &str, data: T)
where
    T: Serialize,
{
    match serde_json::to_string(&data) {
        Ok(json) => info!("{} | data: {}", message, json),
        Err(_) => info!("{} | data: <failed to serialize>", message),
    }
}
