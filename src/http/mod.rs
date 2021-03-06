pub use self::server::{run_http_server, HandlesHttpRequests};
pub use self::client::post;

mod server;
mod client;

#[derive(Debug, PartialEq, Copy)]
pub struct ResponseHeader {
    status: i32,
    content_length: isize,
}

#[derive(Debug, PartialEq)]
pub struct RequestHeader {
    method: String,
    request_uri: String,
    http_version: String,
    content_length: i32,
}

