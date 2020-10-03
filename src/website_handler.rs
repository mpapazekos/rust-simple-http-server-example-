use super::http::{Method, Request, Response, StatusCode};
use super::utils::HttpHandler;

use std::fs;

// =========================================================================

pub struct WebSiteHandler {
    public_path: String,
}

// =========================================================================

impl WebSiteHandler {
    pub fn new(public_path: String) -> Self {
        Self { public_path }
    }

    fn read_file(&self, filepath: &str) -> Option<String> {
        let path = format!("{}/{}", self.public_path, filepath);

        match fs::canonicalize(path) {
            Ok(path) => {
                if path.starts_with(&self.public_path) {
                    return fs::read_to_string(path).ok();
                } else {
                    println!("Directory Traversal Attack Attempted: {}", filepath);
                    return None;
                }
            }

            Err(_) => None,
        }
    }
}

impl HttpHandler for WebSiteHandler {
    fn handle_request(&mut self, request: &Request) -> Response {
        //dbg!(request);

        match request.method() {
            Method::GET => match request.path() {
                "/" => Response::new(StatusCode::Ok, self.read_file("index.html")),
                "/hello" => Response::new(StatusCode::Ok, self.read_file("hello.html")),

                other_path => match self.read_file(other_path) {
                    Some(contents) => Response::new(StatusCode::Ok, Some(contents)),
                    None => Response::new(StatusCode::NotFound, None),
                },
            },

            _ => Response::new(StatusCode::NotFound, None),
        }
    }
}
