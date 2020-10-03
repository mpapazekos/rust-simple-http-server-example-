use super::StatusCode;

use std::fmt::Debug;
use std::io::{Result as IOResult, Write};

// =============================================================

#[derive(Debug)]
pub struct Response {
    status_code: StatusCode,
    body: Option<String>,
}

impl Response {
    pub fn new(status_code: StatusCode, body: Option<String>) -> Self {
        Self { status_code, body }
    }

    pub fn send(&self, tcp_stream: &mut impl Write) -> IOResult<()> {
        let response_body = match &self.body {
            Some(body) => body,
            None => "",
        };

        write!(
            tcp_stream,
            "HTTP/1.1 {} {}\r\n\r\n{}",
            &self.status_code,
            &self.status_code.reason_phrase(),
            response_body
        )
    }
}
