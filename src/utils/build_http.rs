use crate::websocket_types::ResponseStruct;

pub fn build_response(data: ResponseStruct) -> String {
    let mut response_string: String = format!("{}\r\n", data.status.as_str());
    for (key, value) in data.headers {
        response_string.push_str(&format!("{}: {}", key, value));
    }
    response_string.push_str("\r\n");
    response_string
}
