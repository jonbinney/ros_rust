use std::io::TcpStream;

use xmlrpc::parser;
use xmlrpc::{Request, Response, Value};

pub struct Client {
    pub server_uri: String
}

impl Client {
    pub fn execute_request(&self, request: &Request) -> Result<Response, String> {

        let mut stream = match TcpStream::connect(self.server_uri.as_slice()) {
            Ok(x) => x,
            Err(_) => return Err("Unable to connect to xmlrpc server".to_string()),
        };

        let message = create_http_post(try!(serialize_request(request)).as_slice());
        println!("Request: \n{}", message);

        // Send request to server
        match stream.write(message.as_bytes()) {
            Ok(_) => (),
            Err(err) => panic!("{}", err),
        }

        // Read response from server
        let response_str = match stream.read_to_string() {
            Ok(response_str) => response_str,
            Err(err) => panic!("{}", err),
        };

        println!("Response: \n{}", response_str);

        // Parse response
        match parser::parse_response(response_str.as_slice()) {
            Ok(response) => Ok(response),
            Err(err) => Err(err)
        }
    }
}

fn create_http_post(body: &str) -> String {
    format!(
        "POST /RPC2 HTTP/1.0\n\
        User-Agent: RosRust/0.0\n\
        Host: localhost\n\
        Content-Type: text/xml\n\
        Content-length: {content_length}\n\n{body}", content_length=body.len(), body=body)
}

fn serialize_request(request: &Request) -> Result<String, String> {
    let mut param_str = "".to_string();
    for param in request.params.iter() {
        match param {
            &Value::String(ref val) => {
                param_str = param_str + format!(
                  "<param><value><string>{}</string></value></param>", val).as_slice();
            },
            other_val => return Err(format!("Don't know how to serialize XMLRPC value {}", other_val)),
        };
    };

    Ok(format!(
    "<?xml version=\"1.0\"?>\n\
    <methodCall>\n\
    <methodName>{}</methodName>\n\
    <params>\n\
      <param>\n\
      {}\n\
      </param>\n\
    </params>\n\
    </methodCall>\n", request.method_name, param_str))
}

