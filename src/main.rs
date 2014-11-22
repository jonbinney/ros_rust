use std::io::TcpStream;
use std::fmt;
use std::string;

struct ROSNode {
    master_uri: String
}


fn execute_xmlrpc_request(server_uri: &str, request: &str) -> String {

    let mut stream = TcpStream::connect(server_uri).unwrap();

    let header = format!(
        "POST /RPC2 HTTP/1.0\n\
        User-Agent: RosRust/0.0\n\
        Host: localhost\n\
        Content-Type: text/xml\n\
        Content-length: {content_length}\n\n", content_length=request.len());

    let message = header + request.to_string();

    println!("request: {}", message);

    stream.write(message.as_bytes());
    stream.read_to_string().unwrap()
}

impl ROSNode {
    fn get_published_topics(&self) -> String
    {
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

        execute_xmlrpc_request(self.master_uri.as_slice(), request.as_slice())
    }
}

fn main() {
    println!("Starting node");
    let node = ROSNode {master_uri: String::from_str("localhost:11311")};

    let response = node.get_published_topics();
    println!("response: {}", response);
}

