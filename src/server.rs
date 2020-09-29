use crate::http::{ Request, Response, StatusCode, ParseError};

use std::io::Read;
use std::net::TcpListener;
use std::convert::TryFrom;

// =========================================================================

pub trait HttpHandler {

    fn handle_request(&mut self, request: &Request) -> Response;

    fn handle_bad_request(&mut self, error: &ParseError) -> Response {

        println!("Error: failed to parse a request {}", error);
        Response::new(StatusCode::BAD_REQUEST, None)
    }
}

// =========================================================================

pub struct Server {
    
    addr: String,
}

// =========================================================================

impl Server {

    pub fn new(addr: String) -> Self {

        Self{ addr }
    }

    pub fn run(self, mut handler: impl HttpHandler) {


        println!("\nListening on: {}", self.addr);
        println!("~~~~~~~~~~~~~~~~~~~~~~~~~~~~~");

        let tcp_listener = TcpListener::bind(&self.addr).unwrap();

        loop {
                
            match tcp_listener.accept() {

                Ok((mut tcp_stream, addr)) => {

                    println!("================================");
                    println!("New client: {}", addr);
                    println!("================================");
        
                    // not recommended way for production
                    // okay for an example such as this
                    let mut buffer = [0; 1024];

                    match tcp_stream.read(&mut buffer) {

                        Ok(_) => {

                            println!("\tReceived a request\n{}", String::from_utf8_lossy(&buffer));

                            // [..] === slice contains the entire array 
                            let response = match Request::try_from(&buffer[..]) {

                                Ok(request) =>  handler.handle_request(&request),
                                    
                                Err(e) => handler.handle_bad_request(&e)
                            };

                            // Send the response and print msg in case operation fails
                            if let Err(e) = response.send(&mut tcp_stream) {
                                
                                println!("Error: failed to send response {}", e)
                            }
                        }

                        Err(e) => println!("Error: failed to read from connection {}",e)
                    }     
                }
                
                Err(e) => println!("Error: couldn't get client: {}", e) 
            }
        }
    }
}