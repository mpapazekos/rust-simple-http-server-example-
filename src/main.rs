mod website_handler;
mod server;
mod http;

use std::env;
use server::Server;
use website_handler::WebSiteHandler;

// ===================================================

fn main() {
    
    let default_path = format!("{}/public", env!("CARGO_MANIFEST_DIR"));
    let public_path = env::var("PUBLIC_PATH").unwrap_or(default_path);

    let server = Server::new("127.0.0.1:8080".to_string());
    server.run(WebSiteHandler::new(public_path));
}
