extern crate env_logger;
#[macro_use]
extern crate log;

extern crate simple_server;

use simple_server::Server;
use std::env;

fn main() {
    env_logger::init();

    let host = "127.0.0.1";
    let port = match env::var("PORT") {
        Ok(p) => p,
        Err(e) => panic!("couldn't interpret {}: {}", key, e),
    };

    let server = Server::new(|request, mut response| {
        info!("Request received. {} {}", request.method(), request.uri());
        Ok(response.body("Hello Rust!".as_bytes().to_vec())?)
    });

    server.listen(host, port.as_str());
}
