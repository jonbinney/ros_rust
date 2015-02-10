use std::old_io::TcpStream;
use http::ResponseHeader;

fn create_http_post(body: &str) -> String {
    format!(
        "POST /RPC2 HTTP/1.0\n\
        User-Agent: RosRust/0.0\n\
        Host: localhost\n\
        Content-Type: text/xml\n\
        Content-Length: {content_length}\n\n{body}", content_length=body.len(), body=body)
}

fn read_http_response_header<R: Reader>(stream: &mut R) -> Result<ResponseHeader, String> {
    let mut header = ResponseHeader {status: -1, content_length: -1};

    // Read until finding empty line at end of header
    let mut header_str = String::new();
    let mut done = false;
    while !done {
        let b = match stream.read_byte() {
            Ok(b) => b,
            Err(_) => return Err("Failed to read response header from stream".to_string()),
        };
        header_str.push(b as char);
        if header_str.len() >= 4 {
            if header_str.as_slice()[header_str.len()-4..] == *"\r\n\r\n".as_slice() {
                done = true;
            }
        }
    }

    // Parse the status line
    let status_line_re = regex!("^(.+) (.+) (.+)\n");
    let caps = match status_line_re.captures(header_str.as_slice()) {
        None => return Err("Bad status line in response header".to_string()),
        Some(caps) => caps,
    };
    header.status =  match caps.at(2) {
        None => panic!("Missing required field in capture"),
        Some(s) => match s.parse() {
            Err(_) => return Err(format!("Status field in header cannot be parsed to integer ({})", s)),
            Ok(x) => x,
        },
    };

    // Look for the Content-Length
    let content_length_re = regex!("(?i)Content-Length: ([0-9]+)\r\n");
    let caps = match content_length_re.captures(header_str.as_slice()) {
        None => return Err(format!("Header missing Content-Length field:\n{}", header_str)),
        Some(caps) => caps,
    };
    header.content_length = match caps.at(1) {
        None => panic!("Missing required field in capture"),
        Some(s) => match s.parse() {
            Err(_) => return Err(format!("Content-Length field in header cannot be parsed to integer ({})", s)),
            Ok(x) => x,
        },
    };

    Ok(header)
}

/// Read an HTTP response from a stream.
fn read_http_response<R: Reader>(stream: &mut R) -> Result<(ResponseHeader, String), String> {
    let header = match read_http_response_header(stream) {
        Ok(h) => h,
        Err(e) => return Err(format!("Error reading header: {}", e)),
    };

    if header.content_length < 0 {
        return Err(format!("Invalid content-length in header ({})",
            header.content_length));
    };

    let mut body = String::new();
    let mut done = false;
    while !done {
        let b = match stream.read_byte() {
            Ok(b) => b,
            Err(_) => return Err("Failed to read response body from stream".to_string()),
        };
        body.push(b as char);
        if body.len() >= header.content_length as usize {
            done = true;
        }
    }
    Ok((header, body))
}

pub fn post(server_uri: &str, body: &str) -> Result<(ResponseHeader, String), String> {
    // Connect to the server
    let mut stream = match TcpStream::connect(server_uri) {
        Ok(x) => x,
        Err(_) => return Err("Unable to connect to xmlrpc server".to_string()),
    };

    // Create the request
    let message = create_http_post(body);

    // Send request to server
    match stream.write_all(message.as_bytes()) {
        Ok(_) => (),
        Err(_) => return Err("Unable to send request to server".to_string()),
    }

    // Read response from server
    // TODO: Create a BufferedStream for efficiency and set the timeout so that
    // this doesn't hang if the server misbehaves
    read_http_response(&mut stream)
}

#[cfg(test)]
mod tests {
    use http::ResponseHeader;
    use std::old_io::MemReader;

    #[test]
    fn test_parse_response_header() {
        let response_str = "\
        HTTP/1.1 200 OK\r\n\
        Connection: close\r\n\
        Content-Length: 158\r\n\
        Content-Type: text/xml\r\n\r\n";

        let correct_result = ResponseHeader {status: 200, content_length: 158};

        let mut response_reader = MemReader::new(response_str.as_bytes().to_vec());

        match super::read_http_response_header(&mut response_reader) {
            Ok(x) => assert_eq!(x, correct_result),
            Err(err) => assert!(false, err),
        }
    }
}
