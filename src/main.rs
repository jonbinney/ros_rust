use std::io::TcpStream;
use std::fmt;
use std::string;

fn main() {
    println!("Starting node");

    let mut socket = TcpStream::connect("localhost:11311").unwrap();

    let body = " \
    <?xml version=\"1.0\"?> \
    <methodCall> \
      <methodName>getPublishedTopics</methodName> \
      <params> \
        <param> \
            <value><string>meeeee<string></value> \
            <value><string>/<string></value> \
        </param> \
      </params> \
    </methodCall>".to_string();

    let header = format!("POST /RPC2 HTTP/1.0\n \
    User-Agent: Rust-Ros\n \
    Host: localhost\n \
    Content-Type: text/xml\n \
    Content-length: {content_length}\n", content_length=body.len());

    let mut message = header + body;

    println!("request: {}", message);

    socket.write(message.as_bytes());
    let response = socket.read_to_string();
    println!("response: {}", response.unwrap());
}

