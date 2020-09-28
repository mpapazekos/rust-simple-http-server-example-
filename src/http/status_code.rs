use std::fmt::{ Debug, Display, Formatter, Result as FmtResult};

#[derive(Debug, Copy, Clone)]
pub enum StatusCode {

    OK = 200,
    BAD_REQUEST = 400,
    NOT_FOUND = 404
}

impl StatusCode {

    pub fn reason_phrase(&self) -> &str {

        match self {
            
            StatusCode::OK => "Ok",
            StatusCode::BAD_REQUEST => "Bad Request",
            StatusCode::NOT_FOUND => "Not Found",
        }
    }
}

impl Display for StatusCode {   

    fn fmt(&self, f: &mut Formatter) -> FmtResult { 
        write!(f, "{}", *self as u16)     
    }
}