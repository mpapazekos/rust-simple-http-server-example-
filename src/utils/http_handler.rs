use crate::http::{ Request, Response, StatusCode, ParseError};


// =========================================================================

pub trait HttpHandler {

    fn handle_request(&mut self, request: &Request) -> Response;

    fn handle_bad_request(&mut self, error: &ParseError) -> Response {

        println!("Error: failed to parse a request {}", error);
        Response::new(StatusCode::BadRequest, None)
    }
}