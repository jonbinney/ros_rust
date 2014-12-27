use std::io::{TcpListener, TcpStream};
use std::io::{Acceptor, Listener};
use http::RequestHeader;
use std::sync::TaskPool;

pub fn run_http_server<H: HandlesHttpRequests>(
    listener: TcpListener,
    num_threads: uint,
    request_handler: H,
    ) -> Result<(), String> {
    let pool = TaskPool::new(num_threads);

    let mut acceptor = match listener.listen() {
        Ok(x) => x,
        Err(_) => return Err("Failed to create connection acceptor".to_string()),
    };

    for stream in acceptor.incoming() {
        debug!("Got HTTP connection");
        let handler_clone = request_handler.clone();
        match stream {
            Err(_) => {},
            Ok(stream) => pool.execute(move || {
                handle_incoming_request(stream, handler_clone);
            }),
        };
    };

    Ok(())
}

fn handle_incoming_request<H: HandlesHttpRequests>(
    mut stream: TcpStream,
    request_handler: H)
{
    match read_http_request(&mut stream) {
        Ok((header, body)) => {
            let (response_status, response_body) = request_handler.handle_request(&header, body.as_slice());
            let http_response = create_http_response(response_status, response_body.as_slice());
            debug!("Sending response:\n{}", http_response.as_slice());
            match stream.write(http_response.as_bytes()) {
                Ok(_) => (),
                Err(_) => {warn!("Failed to write response");},
            };
        },
        Err(err) => {warn!("Failed to read http request: {}", err);},
    };
}

fn create_http_response(status: int, body: &str) -> String {
    format!("\
        HTTP/1.1 {status} OK\n\
        Connection: close\n\
        Content-Length: {content_length}\n\
        Content-Type: text/xml\n\n{body}", status=status, content_length=body.len(), body=body)
}

fn read_http_request_header<R: Reader>(stream: &mut R) -> Result<RequestHeader, String> {
    let mut header = RequestHeader {
        method: "".to_string(),
        request_uri: "".to_string(),
        http_version: "".to_string(),
        content_length: 0};

    // Read until finding empty line at end of header
    let mut header_str = String::new();
    let mut done = false;
    while !done {
        let b = match stream.read_byte() {
            Ok(b) => b,
            Err(_) => return Err("Failed to read resquest header from stream".to_string()),
        };
        header_str.push(b as char);
        if header_str.len() >= 4 {
            if header_str.as_slice()[header_str.len()-4..] == "\r\n\r\n" {
                done = true;
            }
        }
    }
    debug!("Received header:\n{}", header_str);

    // Parse request line
    let request_line_re = regex!("^(.+) (.+) (.+)\r\n");
    match request_line_re.captures(header_str.as_slice()) {
        None => return Err("Unable to parse header request line".to_string()),
        Some(caps) => {
            header.method = caps.at(1).unwrap().to_string();
            header.request_uri = caps.at(2).unwrap().to_string();
            header.http_version = caps.at(3).unwrap().to_string();
        },
    };

    // Look for the Content-Length if this is a POST
    if header.method.as_slice() == "POST" {
        let content_length_re = regex!("(?i)Content-Length: ([0-9]+)\r\n");
        match content_length_re.captures(header_str.as_slice()) {
            None => return Err("Header missing Content-Length field".to_string()),
            Some(caps) => {
                let s = caps.at(1).unwrap();
                let x: Option<int> = s.parse();
                match x {
                    Some(x) => {
                        header.content_length = x;
                    },
                    None => return Err(format!("Content-Length field in header cannot be parsed to integer ({})", s)),
                };
            },
        };
    };

    Ok(header)
}

/// Read an HTTP request from a stream.
fn read_http_request<R: Reader>(stream: &mut R) -> Result<(RequestHeader, String), String> {
    let header = match read_http_request_header(stream) {
        Ok(h) => h,
        Err(e) => return Err(format!("Error reading header: {}", e)),
    };

    let mut body = String::new();
    match header.content_length {
        0 => {},
        x if x < 0 => {
            return Err(format!("Invalid content-length in header ({})", header.content_length));
        },
        x if x > 0 => {
            // Read request content
            let mut done = false;
            while !done {
                let b = match stream.read_byte() {
                    Ok(b) => b,
                    Err(_) => return Err("Failed to read requst body from stream".to_string()),
                };
                body.push(b as char);
                if body.len() >= header.content_length as uint {
                    done = true;
                }
            };
        },
        _ => return Err("Unexpected value for content_length".to_string()),
    };

    Ok((header, body))
}

pub trait HandlesHttpRequests: Sync + Send + Clone {
    fn handle_request(&self, header: &RequestHeader, body: &str) -> (int, String);
}

#[cfg(test)]
mod tests {
    use http::RequestHeader;
    use std::io::MemReader;

    #[test]
    fn test_parse_request_header() {
        let request_header_str = "\
        POST /RPC2 HTTP/1.0\r\n\
        User-Agent: Frontier/5.1.2 (WinNT)\r\n\
        Host: betty.userland.com\r\n\
        Content-Type: text/xml\r\n\
        Content-Length: 181\r\n\r\n";

        let correct_result = RequestHeader {
            content_length: 181,
            method: "POST".to_string(),
            request_uri: "/RPC2".to_string(),
            http_version: "HTTP/1.0".to_string(),
        };

        let mut request_reader = MemReader::new(request_header_str.as_bytes().to_vec());

        match super::read_http_request_header(&mut request_reader) {
            Ok(x) => assert_eq!(x, correct_result),
            Err(err) => assert!(false, err),
        }
    }
}
