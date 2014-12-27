#![feature(phase)]
#[phase(plugin, link)] extern crate log;
extern crate ros_rust;

use std::io::TcpListener;
use std::sync::{Arc, Mutex};
use ros_rust::http;

#[deriving(Clone)]
struct Handler {
    request_count: Arc<Mutex<int>>,
}

impl http::HandlesHttpRequests for Handler {
    fn handle_request(&self, _: &http::RequestHeader, _: &str) -> (int, String) {
        let mut count = self.request_count.lock();
        let response_body = format!("<html><header></header><body>Hello world {}</body></html>", *count);

        *count = *count + 1;
        (200, response_body)
    }
}

#[allow(dead_code)]
fn main() {
    debug!("Starting server");
    let listener = match TcpListener::bind("127.0.0.1:8080") {
        Ok(l) => l,
        Err(err) => panic!(format!("Unable to bind to port: {}", err)),
    };

    let handler = Handler {request_count: Arc::new(Mutex::new(0))};
    match http::run_http_server(listener, 3, handler) {
        Ok(_) => (),
        Err(_) => panic!("Server died!"),
    }
}
