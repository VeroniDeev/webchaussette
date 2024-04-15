use std::collections::HashMap;

use crate::utils::generate_key::generate_key;

pub struct RequestStruct {
    pub method: String,
    pub uri: String,
    pub headers: HashMap<String, String>,
}

impl Default for RequestStruct {
    fn default() -> Self {
        let mut headers: HashMap<String, String> = HashMap::new();
        headers.insert(String::from("Connection"), String::from("Upgrade"));
        headers.insert(String::from("Upgrade"), String::from("websocket"));
        headers.insert(String::from("Sec-WebSocket-Version"), String::from("13"));
        headers.insert(String::from("Sec-WebSocket-Version"), String::from("13"));
        headers.insert(String::from("Sec-WebSocket-Key"), generate_key());

        Self {
            method: String::new(),
            uri: String::new(),
            headers,
        }
    }
}

pub struct ResponseStruct {
    pub status: String,
    pub headers: HashMap<String, String>,
}
