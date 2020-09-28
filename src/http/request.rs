use super::Method;
use super::ParseError;
use super::QueryString;

use std::str;
use std::convert::TryFrom;
use std::fmt::Debug;

// =============================================================

#[derive(Debug)]
pub struct Request<'buf> {

    path: &'buf str,
    query_string: Option<QueryString<'buf>>,
    method: Method
}

// =============================================================

impl<'buf> Request<'buf> {

    pub fn path(&self) -> &str {&self.path}

    pub fn method(&self) -> &Method {&self.method}

    pub fn query_string(&self) -> Option<&QueryString> {self.query_string.as_ref()}
}

impl<'buf> TryFrom<&'buf [u8]> for Request<'buf> {

    type Error = ParseError;

    // example request:
    // GET/ /search?name=abc&sort=1 HTTP/1.1\r\n...HEADERS...
    fn try_from(buf: &'buf [u8]) -> Result<Self, Self::Error> {

        let request = str::from_utf8(buf)?;

        let (method, request) = get_next_word(request).ok_or(ParseError::InvalidRequest)?;
        let (mut path, request) = get_next_word(request).ok_or(ParseError::InvalidRequest)?;
        let (protocol, _) = get_next_word(request).ok_or(ParseError::InvalidRequest)?;
        
        if protocol != "HTTP/1.1" { return Err(ParseError::InvalidProtocol) }
        
        let method: Method = method.parse()?;

        let mut query_string = None;
        if let Some(idx) = path.find('?') {

            query_string = Some(QueryString::from(&path[idx+1..]));
            path = &path[..idx];
        }  
    
        return Ok(Self {path, query_string, method})
    }
}

fn get_next_word(request: &str) -> Option<(&str, &str)> {

    for (idx, ch) in request.chars().enumerate() {

        if ch == ' ' || ch == '\r'  
        { return Some((&request[..idx], &request[idx+1..])) } 
    }

    return None
}