use httparse::{Header, Request, Status};
use std::{collections::HashMap, error::Error, io::ErrorKind};

use crate::websocket_types::RequestStruct;

fn convert_headers(headers: &[Header<'_>]) -> HashMap<String, String> {
    let mut hashmap: HashMap<String, String> = HashMap::new();

    for header in headers {
        let name: String = header.name.to_string();
        let value: String = String::from_utf8_lossy(header.value).to_string();
        hashmap.insert(name, value);
    }
    hashmap
}

pub fn parse_request(data: String) -> Result<RequestStruct, Box<dyn Error>> {
    let mut request_struct: RequestStruct = RequestStruct::new();
    let mut headers = [httparse::EMPTY_HEADER; 16];
    let mut request = Request::new(&mut headers);

    match request.parse(data.as_ref()) {
        Ok(Status::Complete(_)) => {
            request_struct.method = request.method.unwrap().to_owned();
            request_struct.uri = request.path.unwrap().to_owned();
            request_struct.headers = convert_headers(request.headers);
            Ok(request_struct)
        }
        Ok(Status::Partial) => Err(Box::new(std::io::Error::new(
            ErrorKind::InvalidData,
            "invalid_request",
        ))),
        Err(_) => Err(Box::new(std::io::Error::new(
            ErrorKind::UnexpectedEof,
            "unexpected",
        ))),
    }
}
