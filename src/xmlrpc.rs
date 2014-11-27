use std::io::TcpStream;
use regex::Regex;

pub struct XMLRPCProxy {
    server_uri: String
}

impl XMLRPCProxy {
    pub fn execute_request(&self, request: &str) -> Vec<String> {

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

        // Parse reponse
        parse_response(response_str.as_slice())
    }
}

fn parse_response(response_str: &str) -> Vec<String> {
    let param_re = match Regex::new(r"<value><string>([^<]*)</string></value>") {
        Ok(re) => re,
        Err(err) => panic!("{}", err),
    };

    let mut params: Vec<String> = vec![];
    for cap in param_re.captures_iter(response_str) {
        params.push(cap.at(1).to_string());
        println!("{}", cap.at(1));
    }
    params
}

#[test]
fn test_parse_good_response() {
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

    let params = parse_response(response_str);
    let mut correct_result: Vec<String> = vec![];
    correct_result.push("param1".to_string());
    assert_eq!(params, correct_result);
}

