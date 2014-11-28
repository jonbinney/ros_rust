use std::io::TcpStream;
use regex::Regex;

pub struct XMLRPCProxy {
    server_uri: String
}

impl XMLRPCProxy {
    pub fn execute_request(&self, request: &str) -> Result<Response, String> {

        let mut stream = TcpStream::connect(self.server_uri.as_slice()).unwrap();

        let header = format!(
            "POST /RPC2 HTTP/1.0\n\
            User-Agent: RosRust/0.0\n\
            Host: localhost\n\
            Content-Type: text/xml\n\
            Content-length: {content_length}\n\n", content_length=request.len());

        let message = header + request.to_string();

        println!("request: {}", message);

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

        // Parse response
        match parse_response(response_str.as_slice()) {
            Ok(response) => Ok(response),
            Err(err) => Err(err)
        }
    }
}

#[deriving(Show, PartialEq)]
pub enum XMLRPCValue {
    Empty,
    Int (int),
    Boolean (bool),
    String (String),
    Double (f64),
    // Currently not handling dateTime.iso8601 base64, or struct types
}

/// An XMLRPC response can either be success, in which case a single
/// param is optionally returned, or a fault, in which case a fault code and
/// fault string are optionally included.
#[deriving(Show, PartialEq)]
pub enum Response {
    Success {param: XMLRPCValue},
    Fault {fault_code: int, fault_string: String},
}

fn parse_response(response_str: &str) -> Result<Response, String> {
    let param_re = match Regex::new(r"<value><string>([^<]*)</string></value>") {
        Ok(re) => re,
        Err(err) => return Err(format!("Parse error: {}", err)),
    };

    let mut num_params = 0i;
    let mut param = XMLRPCValue::Empty;
    for cap in param_re.captures_iter(response_str) {
        param = XMLRPCValue::String(cap.at(1).to_string());
        num_params += 1;
    }

    // XMLRPC allows zero or one returned params in a response
    match num_params {
        0|1 => Ok(Response::Success {param: param}),
        _ => Err(format!("Too many parameters in response ({})", num_params)),
    }
}

#[test]
fn test_parse_response_good() {
    let response_str =
    "HTTP/1.1 200 OK\n\
    Content-Length: 158\n\
    Content-Type: text/xml\n\n\
    <?xml version=\"1.0\"?>\n\
    <methodResponse>\n\
       <params>\n\
          <param>\n\
             <value><string>param1</string></value>\n\
          </param>\n\
       </params>\n\
    </methodResponse>\n";

    let response = match parse_response(response_str) {
        Ok(response) => response,
        Err(_) => return assert!(false),
    };
    let correct_response = Response::Success {param: XMLRPCValue::String("param1".to_string())};
    assert_eq!(response, correct_response);
}

#[test]
fn test_parse_response_too_many_params() {
    let response_str =
    "HTTP/1.1 200 OK\n\
    Content-Length: 158\n\
    Content-Type: text/xml\n\n\
    <?xml version=\"1.0\"?>\n\
    <methodResponse>\n\
       <params>\n\
          <param>\n\
             <value><string>param1</string></value>\n\
             <value><string>param2</string></value>\n\
          </param>\n\
       </params>\n\
    </methodResponse>\n";

    match parse_response(response_str) {
        Ok(_) => return assert!(false),
        Err(_) => return (),
    };
}

