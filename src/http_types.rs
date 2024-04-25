pub const HTTP_VERSION: &str = "HTTP/1.1";

pub enum HttpStatus {
    SwitchingProtocols,
    BadRequest,
    Unauthorized,
    Forbidden,
    NotFound,
    InternalServerError,
}

impl HttpStatus {
    pub fn as_str(&self) -> String {
        let status_code = match self {
            HttpStatus::SwitchingProtocols => "101 Switching Protocols",
            HttpStatus::BadRequest => "400 Bad Request",
            HttpStatus::Unauthorized => "401 Unauthorized",
            HttpStatus::Forbidden => "403 Forbidden",
            HttpStatus::NotFound => "404 Not Found",
            HttpStatus::InternalServerError => "500 Internal Server Error",
        };
        let status = format!("{} {}", HTTP_VERSION, status_code);
        status
    }
}
