#![allow(unused_imports, dead_code)]

use handler::WebHandler;
use http::Method;
use http::Request;
use server::Server;
use std::env;
use std::fmt::format;

mod handler;
mod http;
mod server;

fn main() {
    let port = std::env::var("PORT")
        .unwrap_or_else(|_| "8080".to_string())
        .parse()
        .ok();
    let default_path = format!("{}/public", env!("CARGO_MANIFEST_DIR"));
    let public_path = env::var("PUBLIC_PATH").unwrap_or(default_path);
    let server = Server::new(port.unwrap_or(8080));

    server.run(WebHandler::new(public_path));
}
