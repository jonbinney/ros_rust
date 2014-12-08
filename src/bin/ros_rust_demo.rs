extern crate ros_rust;
use ros_rust::xmlrpc;

#[allow(dead_code)]
fn main() {
    println!("Starting node");

    let request = xmlrpc::Request {
        method_name: "getPublishedTopics".to_string(), params: vec![
            xmlrpc::Value::String("meeeee".to_string()),
            xmlrpc::Value::String("/".to_string()),
            ]};

    let c = xmlrpc::client::Client {server_uri: "localhost:11311".to_string()};
    let response = match c.execute_request(&request) {
            Ok(response) => response,
            Err(err) => panic!("Err: {}", err),
        };
    println!("response: {}", response);
}
