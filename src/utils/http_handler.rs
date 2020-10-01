use crate::http::{ Request, Response, StatusCode, ParseError};

use std::net::TcpStream;

// =========================================================================

pub trait HttpHandler {

    fn handle_connection(&mut self, stream: TcpStream);

    fn handle_request(&mut self, request: &Request) -> Response;

    fn handle_bad_request(&mut self, error: &ParseError) -> Response {

        println!("Error: failed to parse a request {}", error);
        Response::new(StatusCode::BAD_REQUEST, None)
    }
}