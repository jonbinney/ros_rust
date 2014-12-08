use xml;
use xmlrpc::{Value, Response};

fn parse_int(s: &str) -> Result<Value, String> {
    let x: Option<int> = from_str(s);
    match x {
        Some(x) => Ok(Value::Int(x)),
        None => Err(format!("String cannot be parsed to integer ({})", s)),
    }
}

/// Parse an XMLRPC data element (e.g. <string>, <int> ...)
fn parse_value_data(element: &xml::Element) -> Result<Value, String> {
    match element.name.as_slice() {
        "i4" => parse_int(element.text.as_slice()),
        //"int" => parse_int(data_element.text),
        //"boolean" => parse_boolean(data_element.text),
        //"string" => parse_string(data_element.text),
        //"double" => parse_double(data_elemnt.text),
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
        name => Err(format!("Expected <value>, found <{}>", name)),
    }
}

/// Parse an XMLRPC response
pub fn parse_response(response_str: &str) -> Result<Response, String> {
    // Technically we should remove the http header first, but the xml
    // parser will actually ignore it and work anyway. Unless there is
    // a "<" in it. Then this will totally fail.
    match xml::parse_xml(response_str) {
        Err(_) => return Err("Failed to parse response xml".to_string()),
        Ok(method_response_element) => match method_response_element.children.len() {
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
        },
    }
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
        Err(_) => return assert!(false),
    };
    let correct_response = Response::Success {param: Value::string("param1".to_string())};
    assert_eq!(response, correct_response);
}

#[test]
fn test_parse_response_too_many_params() {
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

