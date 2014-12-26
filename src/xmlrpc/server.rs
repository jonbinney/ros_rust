use std::io::{TcpListener, TcpStream};
use std::io::{Acceptor, Listener};

//use xmlrpc::parser;
//use xmlrpc::{Request, Response};

pub fn run() -> Result<(), String> {
    let listener = TcpListener::bind("0.0.0.0:1212");

    let mut acceptor = match listener.listen() {
        Ok(x) => x,
        Err(_) => return Err("Failed to create connection acceptor".to_string()),
    };

    for stream in acceptor.incoming() {
        match stream {
            Err(_) => {},
            Ok(stream) => handle_incoming_request(stream),
        }
    }

    Ok(())
}

fn handle_incoming_request(mut stream: TcpStream) {
    // Read response from server
    let request_str = match stream.read_to_string() {
        Ok(response_str) => response_str,
        Err(err) => panic!("{}", err),
    };

    println!("===== Got request: \n{}\n=====", request_str);
}

