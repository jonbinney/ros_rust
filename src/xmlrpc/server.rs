use std::io::{TcpListener, TcpStream};

use xmlrpc::parser;
use xmlrpc::{Request, Response};

pub fn run() -> Result<TcpListener, String> {
    TcpListener::bind("0.0.0.0");

    // bind the listener to the specified address
    let mut acceptor = listener.listen();
}

fn handle_incoming_request() {
}

