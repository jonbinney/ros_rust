#![feature(io)]
extern crate ros_rust;
use ros_rust::xmlrpc;

use std::old_io::TcpListener;

#[derive(Clone)]
struct Handler;

impl xmlrpc::HandlesXmlrpcRequests for Handler {
    fn handle_request(&self, _: &xmlrpc::Request) -> xmlrpc::Response {
        println!("User got XMLRPC request");
        xmlrpc::Response::Success {param: xmlrpc::Value::Boolean(true)}
    }
}

#[allow(dead_code)]
fn main() {
    println!("Starting xmlrpc server");

    // Parameters
    let ros_master_uri = "localhost:11311";
    let node_port: i32 = 1212;
    let caller_api = format!("http://127.0.0.1:{}", node_port);
    println!("caller_api: {}", caller_api);

    println!("Registering published topic");
    let request = xmlrpc::Request {
        method_name: "registerPublisher".to_string(), params: vec![
            xmlrpc::Value::String("/me".to_string()),
            xmlrpc::Value::String("/foo".to_string()),
            xmlrpc::Value::String("std_msgs/String".to_string()),
            xmlrpc::Value::String(caller_api),
            ]};

    let c = xmlrpc::Client {server_uri: ros_master_uri.to_string()};
    let response = match c.execute_request(&request) {
            Ok(response) => response,
            Err(err) => panic!("Err: {}", err),
        };
    println!("response: {:?}", response);

    // Run a ROS Slave XMLRPC server
    println!("Starting server");
    let handler = Handler;
    let listener = match TcpListener::bind(format!("127.0.0.1:{}", node_port).as_slice()) {
        Ok(l) => l,
        Err(err) => panic!(format!("Unable to bind to port: {}", err)),
    };
    match xmlrpc::run_xmlrpc_server(listener, 4, handler) {
        Ok(_) => {println!("Exiting happily")},
        Err(_) => {println!("Exiting on error")}
    };
}

