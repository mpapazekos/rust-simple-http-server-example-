mod http;
mod server;
mod utils;
mod website_handler;

use server::Server;
use utils::ThreadPool;
use website_handler::WebSiteHandler;

use std::env;

// ===================================================

fn main() {
    let default_path = format!("{}/public", env!("CARGO_MANIFEST_DIR"));
    let public_path = env::var("PUBLIC_PATH").unwrap_or(default_path);

    let pool = ThreadPool::new(4);
    let server = Server::new("127.0.0.1:8080".to_string(), public_path, &pool);

    server.run();

    println!("Shutting down.");
}
