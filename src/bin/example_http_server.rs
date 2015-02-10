#![feature(io)]
#[macro_use]
extern crate log;
extern crate ros_rust;

use std::old_io::TcpListener;
use std::sync::{Arc, Mutex};
use ros_rust::http;

#[derive(Clone)]
struct Handler {
    request_count: Arc<Mutex<isize>>,
}

impl http::HandlesHttpRequests for Handler {
    fn handle_request(&self, _: &http::RequestHeader, _: &str) -> (i32, String) {
        let mut count = self.request_count.lock().unwrap();
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
