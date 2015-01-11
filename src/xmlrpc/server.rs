use std::io::TcpListener;
use http;
use xmlrpc::parser;
use xmlrpc::common::{Request, Response, Value};

pub fn run_xmlrpc_server<H: HandlesXmlrpcRequests>(
    listener: TcpListener,
    num_threads: usize,
    xmlrpc_request_handler: H,
    ) -> Result<(), String>
{
    let handler = RequestHandler {xmlrpc_request_handler: xmlrpc_request_handler};

    match http::run_http_server(listener, num_threads, handler) {
        Ok(_) => Ok(()),
        Err(_) => panic!("HTTP server died"),
    }
}

fn serialize_response(response: &Response) -> Result<String, String> {
    match *response {
        Response::Fault {fault_code: _, fault_string: _} => return Err(format!("Don't know how to serialize fault responses")),
        Response::Success {ref param} => {
            let param_str = match param {
                &Value::String(ref val) => format!(
                    "<param><value><string>{}</string></value></param>", val),
                other_val => return Err(format!("Don't know how to serialize XMLRPC value {:?}", other_val)),
            };

            Ok(format!(
                "<?xml version=\"1.0\"?>\n\
                <methodResponse>\n\
                <params>\n\
                  <param>\n\
                  {}\n\
                  </param>\n\
                </params>\n\
                </methodResponse>\n", param_str))
        },
    }
}

/// Handles HTTP requests by parsing out the XMLRPC request, and calling
/// the user supplied callback on it.
#[derive(Clone)]
struct RequestHandler<H: HandlesXmlrpcRequests> {
    xmlrpc_request_handler: H,
}

impl<H: HandlesXmlrpcRequests> http::HandlesHttpRequests for RequestHandler<H> {
    fn handle_request(&self, _: &http::RequestHeader, body: &str) -> (i32, String) {
        println!("==== Got xmlrpc request:\n{}----\n", body);

        match parser::parse_request(body) {
            Err(err) => {
                // TODO: Return a fault response to client
                panic!(format!("Unable to parse incoming xmlrpc request:\n{}", err))
            },
            Ok(request) => {
                let response = self.xmlrpc_request_handler.handle_request(&request);
                match serialize_response(&response) {
                    Err(err) => panic!(format!("Unable to serialize xmlrpc response:\n{}", err)),
                    Ok(response_str) => {
                        println!("response:\n{}====\n", response_str);
                        (200, response_str)
                    },
                }
            },
        }
    }
}

pub trait HandlesXmlrpcRequests: Sync + Send + Clone {
    fn handle_request(&self, request: &Request) -> Response;
}
