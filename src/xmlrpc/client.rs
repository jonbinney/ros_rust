use http::post;
use xmlrpc::parser;
use xmlrpc::{Request, Response, Value};

pub struct Client {
    pub server_uri: String
}

impl Client {
    pub fn execute_request(&self, request: &Request) -> Result<Response, String> {
        let request_str = match serialize_request(request) {
            Err(_) => return Err("Failed to serialize request".to_string()),
            Ok(r) => r,
        };

        match post(self.server_uri.as_slice(), request_str.as_slice()) {
            Err(err) => Err(err),
            Ok((_, response_body)) => match parser::parse_response(response_body.as_slice()) {
                Ok(response) => Ok(response),
                Err(err) => Err(err)
            },
        }
    }
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
      {}\n\
    </params>\n\
    </methodCall>\n", request.method_name, param_str))
}

