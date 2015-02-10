use xml;
use xmlrpc::{Value, Request, Response};

fn parse_int(s: &str) -> Result<Value, String> {
    match s.parse() {
        Ok(x) => Ok(Value::Int(x)),
        Err(_) => Err(format!("String cannot be parsed to integer ({})", s)),
    }
}

fn parse_string(s: &str) -> Result<Value, String> {
    Ok(Value::String(s.to_string()))
}

fn parse_array(element: &xml::Element) -> Result<Value, String> {
    match element.children.len() {
        1 => parse_array_data(&element.children[0]),
        x => Err(format!("Bad number of children for <array> element ({})", x)),
    }
}

fn parse_array_data(element: &xml::Element) -> Result<Value, String> {
    let mut array = vec![];
    for child in element.children.iter() {
        match parse_value(child) {
            Ok(val) => {array.push(val)},
            Err(e) => return Err(e),
        }
    }
    Ok(Value::Array(array))
}

/// Parse an XMLRPC data element (e.g. <string>, <int> ...)
fn parse_value_data(element: &xml::Element) -> Result<Value, String> {
    match element.name.as_slice() {
        "i4" => parse_int(element.text.as_slice()),
        "int" => parse_int(element.text.as_slice()),
        //"boolean" => parse_boolean(data_element.text),
        "string" => parse_string(element.text.as_slice()),
        //"double" => parse_double(element.text),
        "array" => parse_array(element),
        // Currently not handling dateTime.iso8601 base64 or struct types
        x => Err(format!("Found unknown xmlrpc datatype ({})", x)),
    }
}

/// Parse an XMLRPC <value> element
fn parse_value(element: &xml::Element) -> Result<Value, String> {
    match element.name.as_slice() {
        "value" => match element.children.len() {
            1 => parse_value_data(&element.children[0]),
            x => Err(format!("Bad number of children for <value> element ({})", x)),
        },
        name => Err(format!("Expected <value>, found <{}>", name)),
    }
}

/// Parse an XMLRPC <param> element
fn parse_param(element: &xml::Element) -> Result<Value, String> {
    match element.name.as_slice() {
        "param" => match element.children.len() {
            1 => parse_value(&element.children[0]),
            x => Err(format!("Bad number of children for <param> element ({})", x)),
        },
        name => Err(format!("Expected <param>, found <{}>", name)),
    }
}

/// Parse an XMLRPC request
pub fn parse_request(request_str: &str) -> Result<Request, String> {
    let request_element = try!(xml::parse_xml(request_str));

    if request_element.name != "methodCall" {
        return Err(format!("Expected methodCall element; found {}", request_element.name));
    }

    let mut found_method_name = false;
    let mut request = Request {method_name: "".to_string(), params: vec![]};
    for child in request_element.children.iter() {
        match child.name.as_slice() {
            "methodName" => {
                found_method_name = true;
                request.method_name = child.text.clone();
            },
            "params" => {
                for param_element in child.children.iter() {
                    match parse_param(param_element) {
                        Ok(x) => {
                            request.params.push(x);
                        },
                        Err(err) => {
                            return Err(format!("Error parsing param in request: {}", err));
                        }
                    }
                }
            },
            x => {
                return Err(format!("Unexpected element in methodCall: {}", x));
            },
        }
    }

    match found_method_name {
        true => Ok(request),
        false => Err("Request missing methodName element".to_string()),
    }
}

/// Parse an XMLRPC response
pub fn parse_response(response_str: &str) -> Result<Response, String> {
    let method_response_element = try!(xml::parse_xml(response_str));
    match method_response_element.children.len() {
        1 => match method_response_element.children[0] {
            ref params_element => match params_element.children.len() {
                1 => match parse_param(&params_element.children[0]) {
                    Ok(x) => Ok(Response::Success {param: x}),
                        Err(err) => return Err(err),
                },
                x => return Err(format!("Bad number of params in XMLRPC response ({})", x)),
            },
        },
          x => return Err(format!("Bad number of children for methodResponse ({})", x)),
    }
}

#[cfg(test)]
mod tests {
    use xml;
    use xmlrpc::{Request, Response, Value};
    use super::{parse_request, parse_response, parse_value};

    #[test]
    fn test_request_good() {
        let request_str= "\
        <?xml version=\"1.0\"?>\n\
        <methodCall>\n\
          <methodName>foo</methodName>\n\
          <params>\n\
            <param>\n\
              <value><i4>42</i4></value>\n\
            </param>\n\
          </params>\n\
        </methodCall>\n";

        let request = match parse_request(request_str) {
            Ok(request) => request,
            Err(err) => return assert!(false, "Parsing of request failed: {}", err),
        };
        let correct_request = Request {method_name: "foo".to_string(), params: vec![Value::Int(42)]};
        assert_eq!(request, correct_request);
    }

    #[test]
    fn test_parse_response_good() {
        let response_str =
        "<?xml version=\"1.0\"?>\n\
        <methodResponse>\n\
           <params>\n\
              <param>\n\
                 <value><string>param1</string></value>\n\
              </param>\n\
           </params>\n\
        </methodResponse>\n";

        let response = match parse_response(response_str) {
            Ok(response) => response,
            Err(err) => return assert!(false, "Parsing of response failed: {}", err),
        };
        let correct_response = Response::Success {param: Value::String("param1".to_string())};
        assert_eq!(response, correct_response);
    }

    #[test]
    fn test_parse_response_too_many_values() {
        let response_str =
        "<?xml version=\"1.0\"?>\n\
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

    #[test]
    fn test_parse_array_simple() {
        let array_str =
        "<value><array><data>\n\
        <value><int>1</int></value>\n\
        <value><int>2</int></value>\n\
        </data></array></value>\n";

        let correct_val = Value::Array(vec![Value::Int(1), Value::Int(2)]);

        let value_element = match xml::parse_xml(array_str) {
            Ok(el) => el,
            Err(err) => return assert!(false, err),
        };

        match parse_value(&value_element) {
            Ok(val) => return assert_eq!(val, correct_val),
            Err(err) => return assert!(false, err),
        };
    }

    #[test]
    fn test_parse_array_nested() {
        let array_str =
        "<value><array><data>\n\
        <value><int>1</int></value>\n\
        <value><string>Registered [meeeee] as publisher of [/foo]</string></value>\n\
        <value><array><data>\n\
        </data></array></value>\n\
        </data></array></value>\n";

        let correct_val = Value::Array(
            vec![Value::Int(1),
            Value::String("Registered [meeeee] as publisher of [/foo]".to_string()),
            Value::Array(vec![])]);

        let value_element = match xml::parse_xml(array_str) {
            Ok(el) => el,
            Err(err) => return assert!(false, err),
        };

        match parse_value(&value_element) {
            Ok(val) => return assert_eq!(val, correct_val),
            Err(err) => return assert!(false, err),
        };
    }
}

