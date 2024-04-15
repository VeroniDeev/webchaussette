use std::error::Error;

use crate::{
    utils::generate_accept,
    websocket_types::{RequestStruct, ResponseStruct},
};

pub fn create_response(request: RequestStruct) -> Result<ResponseStruct, Box<dyn Error>> {
    let mut response: ResponseStruct = ResponseStruct::default();

    if let Some(key) = request.headers.get("key").cloned() {
        response
            .headers
            .insert(String::from("Sec-WebSocket-Accept"), generate_accept(key));
    }

    Ok(response)
}
