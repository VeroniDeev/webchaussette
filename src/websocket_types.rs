use std::collections::HashMap;

use crate::{http_types::HttpStatus, utils::generate_key};

pub const WEBSOCKET_GUID: &str = "258EAFA5-E914-47DA-95CA-C5AB0DC85B11";
const WEBSOCKET_VERSION: &str = "13";
pub const BUFFER_SIZE: usize = 1024;

#[derive(Debug, Clone)]
pub struct RequestStruct {
    pub method: String,
    pub uri: String,
    pub headers: HashMap<String, String>,
}

impl RequestStruct {
    pub fn new() -> Self {
        Self {
            method: String::new(),
            uri: String::new(),
            headers: HashMap::new(),
        }
    }

    fn default() -> Self {
        let mut headers: HashMap<String, String> = HashMap::new();
        headers.insert(String::from("Connection"), String::from("Upgrade"));
        headers.insert(String::from("Upgrade"), String::from("websocket"));
        headers.insert(
            String::from("Sec-WebSocket-Version"),
            String::from(WEBSOCKET_VERSION),
        );
        headers.insert(String::from("Sec-WebSocket-Key"), generate_key());

        Self {
            method: String::new(),
            uri: String::new(),
            headers,
        }
    }
}

pub struct ResponseStruct {
    pub status: HttpStatus,
    pub headers: HashMap<String, String>,
}

impl Default for ResponseStruct {
    fn default() -> Self {
        let mut headers: HashMap<String, String> = HashMap::new();
        headers.insert(String::from("Connection"), String::from("Upgrade"));
        headers.insert(String::from("Upgrade"), String::from("websocket"));

        Self {
            status: HttpStatus::SwitchingProtocols,
            headers,
        }
    }
}
