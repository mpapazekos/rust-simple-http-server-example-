use super::method::MethodError;

use std::str::Utf8Error;
use std::error::Error;
use std::fmt::{ Debug, Display, Formatter, Result as FmtResult};

// ===========================================================

pub enum ParseError {

    InvalidMethod,
    InvalidRequest,
    InvalidProtocol,
    InvalidEncoding
}

// ===========================================================

impl ParseError {

    fn message(&self) -> &str {

        match self {

            Self::InvalidMethod => "InvalidMethod",
            Self::InvalidRequest => "InvalidRequest",        
            Self::InvalidProtocol => "InvalidProtocol",
            Self::InvalidEncoding => "InvalidEncoding" 
        }
    }
}


impl From<Utf8Error> for ParseError {

    fn from(_: Utf8Error) -> Self { Self::InvalidEncoding }
}

impl From<MethodError> for ParseError {

    fn from(_: MethodError) -> Self { Self::InvalidMethod }
}

impl Display for ParseError {

    fn fmt(&self, f: &mut Formatter) -> FmtResult { write!(f, "{}", self.message()) }
}

impl Debug for ParseError {

    fn fmt(&self, f: &mut Formatter) -> FmtResult { write!(f, "{}", self.message()) }
}

impl Error for ParseError {}

