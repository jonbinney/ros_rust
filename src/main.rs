use std::io::TcpStream;
use std::fmt;
use std::string;

fn execute_xmlrpc_request(stream: &mut TcpStream, request: &String) -> String {

    let header = format!(
        "POST /RPC2 HTTP/1.0\n\
        User-Agent: RosRust/0.0\n\
        Host: localhost\n\
        Content-Type: text/xml\n\
        Content-length: {content_length}\n\n", content_length=request.len());

    let message = header + *request;

    println!("request: {}", message);

    stream.write(message.as_bytes());
    stream.read_to_string().unwrap()
}

fn main() {
    println!("Starting node");

    let mut stream = TcpStream::connect("localhost:11311").unwrap();

    let request =
        "<?xml version=\"1.0\"?>\n\
        <methodCall>\n\
          <methodName>getPublishedTopics</methodName>\n\
          <params>\n\
            <param>\n\
                <value><string>meeeee</string></value>\n\
                <value><string></string></value>\n\
            </param>\n\
          </params>\n\
        </methodCall>\n".to_string();

    let response = execute_xmlrpc_request(&mut stream, &request);
    println!("response: {}", response);
}

