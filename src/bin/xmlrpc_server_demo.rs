extern crate ros_rust;
use ros_rust::xmlrpc;

#[allow(dead_code)]
fn main() {
    println!("Starting xmlrpc server");

    // Parameters
    let ros_master_uri = "localhost:11311";
    let node_port: int = 1212;
    let caller_api = format!("http://127.0.0.1:{}", node_port);
    println!("caller_api: {}", caller_api);

    println!("Registering published topic");
    let request = xmlrpc::Request {
        method_name: "registerPublisher".to_string(), params: vec![
            xmlrpc::Value::String("meeeee".to_string()),
            xmlrpc::Value::String("/foo".to_string()),
            xmlrpc::Value::String("std_msgs/String".to_string()),
            xmlrpc::Value::String(caller_api),
            ]};

    let c = xmlrpc::client::Client {server_uri: ros_master_uri.to_string()};
    let response = match c.execute_request(&request) {
            Ok(response) => response,
            Err(err) => panic!("Err: {}", err),
        };
    println!("response: {}", response);
}
