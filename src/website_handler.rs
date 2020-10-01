use super::utils::HttpHandler;
use super::http::{Request, Response, StatusCode, Method};

use std::fs;
use std::io::Read;
use std::convert::TryFrom;
use std::net::TcpStream;

// =========================================================================

pub struct WebSiteHandler {

    public_path: String 
}

// =========================================================================

impl WebSiteHandler {

    pub fn new(public_path: String) -> Self {

        Self { public_path }
    }

    fn read_file(&self, filepath: &str) -> Option<String> {

        let path = format!("{}/{}",self.public_path, filepath);

        match fs::canonicalize(path) {

            Ok(path) => {

                if path.starts_with(&self.public_path) {

                    return fs::read_to_string(path).ok()
                }
                else {
                    println!("Directory Traversal Attack Attempted: {}", filepath);
                    return None
                }
            }
            
            Err(_) => None
        }
    }
}


impl HttpHandler for WebSiteHandler {

    fn handle_connection(&mut self, mut stream: TcpStream) {

        // not recommended way for production
        // okay for an example such as this
        let mut buffer = [0; 1024];
    
        match stream.read(&mut buffer) {
    
            Ok(_) => {
    
                println!("\tReceived a request\n{}", String::from_utf8_lossy(&buffer));
    
                // [..] === slice contains the entire array 
                let response = match Request::try_from(&buffer[..]) {
    
                    Ok(request) =>  self.handle_request(&request),
                         
                    Err(e) => self.handle_bad_request(&e)
                };
    
                // Send the response and print msg in case operation fails
                if let Err(e) = response.send(&mut stream) {
                    
                    println!("Error: failed to send response {}", e)
                }
            }
    
            Err(e) => println!("Error: failed to read from connection {}",e)
        }     
    }

    fn handle_request(&mut self, request: &Request) -> Response {

        dbg!(request);

        match request.method() { 
            
            Method::GET => match request.path() {

                "/"      => Response::new(StatusCode::OK, self.read_file("index.html")),
                "/hello" => Response::new(StatusCode::OK, self.read_file("hello.html")),
                
                other_path => match self.read_file(other_path) {

                    Some(contents) => Response::new(StatusCode::OK, Some(contents)),
                    None => Response::new(StatusCode::NOT_FOUND, None)
                }
                    
            }
            
            _=> Response::new(StatusCode::NOT_FOUND, None) 
        }
    }
}
