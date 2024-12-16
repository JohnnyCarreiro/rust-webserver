use super::status_code::StatusCode;
use std::io::Write;
use std::{
    fmt::{Display, Formatter, Result as FormatResult},
    io::Result as IoResult,
};

pub struct Response {
    status_code: StatusCode,
    body: Option<String>,
}

impl Response {
    pub fn new(status_code: StatusCode, body: Option<String>) -> Self {
        Self { status_code, body }
    }
    //     pub fn send(&self, stream: &mut dyn Write) -> IoResult<()> { dyn will resolve at
    //     runtime, impl instead will resolve at compile time
    pub fn send(&self, stream: &mut impl Write) -> IoResult<()> {
        let body = match &self.body {
            Some(b) => b,
            None => "",
        };
        write!(
            stream,
            "HTTP/1.1 {} {}\r\n\r\n{}",
            self.status_code,
            self.status_code.reason_phrase(),
            body
        )
    }
}

impl Display for Response {
    fn fmt(&self, f: &mut Formatter<'_>) -> FormatResult {
        let body = match &self.body {
            Some(b) => b,
            None => "",
        };
        write!(
            f,
            "HTTP/1.1 {} {}\r\n\r\n{}",
            self.status_code,
            self.status_code.reason_phrase(),
            body
        )
    }
}
