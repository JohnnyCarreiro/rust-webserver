use super::method::{Method, MethodError};
use super::QueryString;
use std::convert::TryFrom;
use std::error::Error;
use std::fmt::{Debug, Display, Formatter, Result as FormatResult};
use std::str;
use std::str::Utf8Error;

#[derive(Debug)]
pub struct Request<'buf> {
    path: &'buf str,
    query_string: Option<QueryString<'buf>>,
    method: Method,
}

impl<'buf> Request<'buf> {
    pub fn path(&self) -> &str {
        self.path
    }

    pub fn method(&self) -> &Method {
        &self.method
    }

    pub fn query_string(&self) -> Option<&QueryString> {
        self.query_string.as_ref()
    }
}

impl<'buf> TryFrom<&'buf [u8]> for Request<'buf> {
    type Error = ParseReqError;

    fn try_from(buf: &'buf [u8]) -> Result<Self, Self::Error> {
        let req = str::from_utf8(buf)?;
        let (method, req) = get_next_word(req).ok_or(ParseReqError::InvalidRequest)?;
        let (mut path, req) = get_next_word(req).ok_or(ParseReqError::InvalidRequest)?;
        let (protocol, _) = get_next_word(req).ok_or(ParseReqError::InvalidRequest)?;

        if protocol != "HTTP/1.1" {
            return Err(ParseReqError::InvalidProtocol);
        }

        let method: Method = method.parse()?;
        let mut query_string = None;

        if let Some(i) = path.find('?') {
            query_string = Some(QueryString::from(&path[i + 1..]));
            path = &path[..i];
        }

        Ok(Self {
            path,
            method,
            query_string,
        })
    }
}

fn get_next_word(text: &str) -> Option<(&str, &str)> {
    for (i, c) in text.chars().enumerate() {
        if c == ' ' || c == '\r' {
            return Some((&text[..i], &text[i + 1..]));
        }
    }
    None
}

pub enum ParseReqError {
    InvalidRequest,
    InvalidEncoding,
    InvalidProtocol,
    InvalidMethod,
}

impl ParseReqError {
    fn message(&self) -> &str {
        match self {
            Self::InvalidRequest => "Invalid Request",
            Self::InvalidEncoding => "Invalid Encoding",
            Self::InvalidProtocol => "Invalid Protocol",
            Self::InvalidMethod => "Invalid Method",
        }
    }
}

impl From<Utf8Error> for ParseReqError {
    fn from(_: Utf8Error) -> Self {
        Self::InvalidEncoding
    }
}

impl From<MethodError> for ParseReqError {
    fn from(_: MethodError) -> Self {
        Self::InvalidMethod
    }
}

impl Display for ParseReqError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FormatResult {
        write!(f, "{}", self.message())
    }
}

impl Debug for ParseReqError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FormatResult {
        write!(f, "{}", self.message())
    }
}

impl Error for ParseReqError {}

// impl TryFrom<&[u8]> for Request {
//     type Error = ParseReqError;
//
//     fn try_from(buf: &[u8]) -> Result<Self, Self::Error> {
//         // match str::from_utf8(buf) {
//         //     Ok(request) => {}
//         //     Err(_) => return Err(ParseReqError::InvalidEncoding),
//         // }
//         //
//         // match str::from_utf8(buf).or(Err(ParseReqError::InvalidEncoding)) {
//         //     Ok(request) => {}
//         //     Err(e) => return Err(e),
//         // }
//         // // this represents the code above, bur using a nice syntax sugar;
//         // let req = str::from_utf8(buf).or(Err(ParseReqError::InvalidEncoding))?;
//
//         let req = str::from_utf8(buf)?;
//         match get_next_word(req) {
//     Some((method, req)) => match get_next_word(req) {
//
//     }
//     None => return Err(ParseReqError::InvalidRequest),
// }
//         todo!()
//     }
// }
