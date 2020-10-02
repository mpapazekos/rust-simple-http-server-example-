use crate::utils::{ HttpHandler, ThreadPool};
use crate::WebSiteHandler;
use crate::http::Request;

use std::io::Read;
use std::sync::Arc;
use std::convert::TryFrom;
use std::net::{TcpListener, TcpStream};

// =========================================================================

pub struct Server<'s> {
    
    addr: String,
    public_path: String,
    thread_pool: &'s ThreadPool,
}

// =========================================================================

impl<'s> Server<'s> {

    pub fn new(addr: String, public_path: String, thread_pool: &'s ThreadPool) -> Self {

        Self{ addr, public_path, thread_pool }
    }

    pub fn run(self) {

        println!("\nListening on: {}", self.addr);
        println!("~~~~~~~~~~~~~~~~~~~~~~~~~~~~~");

        let tcp_listener = TcpListener::bind(&self.addr).unwrap();
        let arc_path = Arc::new(self.public_path);
        

        // keep listening for new connections
        loop {
            
            

            // when a connection is found 
            match tcp_listener.accept() {

                Ok((stream, addr)) => {

                    println!("==================================================");
                    println!("\t\t New client: {}", addr);
                    println!("==================================================");
                    
                    let cloned_path = Arc::clone(&arc_path).to_string();

                    &self.thread_pool.assign_mission(move || {

                        let handler = WebSiteHandler::new(cloned_path);

                        mission_details(handler, stream);
                    });
                    
                   
                }
                
                Err(e) => println!("Error: couldn't get client: {}", e) 
            }
        }
    }
}

fn mission_details(mut handler: impl HttpHandler, mut stream: TcpStream) {

    // not recommended way for production
    // okay for an example such as this
    let mut buffer = [0; 1024];

    match stream.read(&mut buffer) {

        Ok(_) => {

            println!("\n-----------------------------");
            println!("\tReceived a request");
            println!("-----------------------------");

            println!("{}", String::from_utf8_lossy(&buffer));
            println!("\n^^^^^^^^^^^^^^^^");

            // [..] === slice contains the entire array 
            let response = match Request::try_from(&buffer[..]) {

                Ok(request) =>  handler.handle_request(&request),
                    
                Err(e) => handler.handle_bad_request(&e)
            };

            // Send the response and print msg in case operation fails
            if let Err(e) = response.send(&mut stream) {
                
                println!("Error: failed to send response {}", e)
            }
        }

        Err(e) => println!("Error: failed to read from connection {}",e)
    }     
}
