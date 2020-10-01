use crate::utils::HttpHandler;

use std::net::TcpListener;

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

        // keep listening for new connections
        loop {
            
            // when a connection is found 
            match tcp_listener.accept() {

                Ok((mut stream, addr)) => {

                    println!("================================");
                    println!("New client: {}", addr);
                    println!("================================");
        
                    handler.handle_connection(stream);
                }
                
                Err(e) => println!("Error: couldn't get client: {}", e) 
            }
        }
    }
}

