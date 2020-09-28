use std::str::FromStr;

#[derive(Debug)]
pub enum Method {
    GET,
    POST,
    PUT,
    DELETE,
    HEAD,
    TRACE,
    PATCH,
    CONNECT,
    OPTIONS
}

impl FromStr for Method {

    type Err = MethodError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        
        match s {
            "GET"  => Ok(Self::GET),
            "POST" => Ok(Self::POST),
            "PUT"  => Ok(Self::PUT),
            "DELETE" => Ok(Self::DELETE),
            "HEAD"   => Ok(Self::HEAD),
            "TRACE"  => Ok(Self::TRACE),
            "PATCH"  => Ok(Self::PATCH),
            "CONNECT" => Ok(Self::CONNECT),
            "OPTIONS" => Ok(Self::OPTIONS),
            _ => Err(MethodError)
        }
    }
}

// =====================================================

pub struct MethodError;