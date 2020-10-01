use super::Method;
use super::ParseError;
use super::QueryString;
use super::HttpHeaderMap;

use std::str;
use std::convert::TryFrom;
use std::fmt::Debug;

// =============================================================

#[derive(Debug)]
pub struct Request<'buf> {

    method: Method,
    path: &'buf str,
    query_string: Option<QueryString<'buf>>,
    headers: Option<HttpHeaderMap<'buf>>
}

// =============================================================

impl<'buf> Request<'buf> {

    pub fn path(&self) -> &str {&self.path}

    pub fn method(&self) -> &Method {&self.method}

    pub fn http_headers(&self) -> Option<&HttpHeaderMap> { self.headers.as_ref()}
    
    pub fn query_string(&self) -> Option<&QueryString> {self.query_string.as_ref()}
}

impl<'buf> TryFrom<&'buf [u8]> for Request<'buf> {

    type Error = ParseError;

    // example request:
    // GET / /search?name=abc&sort=1 HTTP/1.1\r\n...HEADERS...
    fn try_from(buf: &'buf [u8]) -> Result<Self, Self::Error> {

        let request = str::from_utf8(buf)?;

        // request breakdown
        // -------------------
        let (method, request)   = get_next_phrase(request).ok_or(ParseError::InvalidRequest)?;
        let (mut path, request) = get_next_phrase(request).ok_or(ParseError::InvalidRequest)?;
        let (protocol, request) = get_next_phrase(request).ok_or(ParseError::InvalidRequest)?;
        let (_, request)        = get_next_phrase(request).ok_or(ParseError::InvalidRequest)?;

        // check protocol
        // -------------------
        if protocol != "HTTP/1.1" {

            return Err(ParseError::InvalidProtocol) 
        }
        
        // check method 
        // -------------------
        let method: Method = method.parse()?;

        // check path and query string
        // -------------------
        let mut query_string = None;

        if let Some(idx) = path.find('?') {

            query_string = Some(QueryString::from(&path[idx+1..]));
            path = &path[..idx];
        }  

        // check http headers
        // -------------------
        let mut headers = None;

        if let Some(idx) = request.find("\r\n\r\n") {

            headers = Some(HttpHeaderMap::from(&request[..idx]));
        }

        return Ok(Self {method, path, query_string, headers})
    }
}

fn get_next_phrase(request: &str) -> Option<(&str, &str)> {

    for (idx, ch) in request.chars().enumerate() {

        if ch == ' ' || ch == '\r' || ch == '\n' 
        { return Some((&request[..idx], &request[idx+1..])) } 
    }

    return None
}