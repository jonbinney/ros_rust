use xml;
use xmlrpc::{Value, Response};

fn parse_int(s: &str) -> Result<Value, String> {
    let x: Option<int> = from_str(s);
    match x {
        Some(x) => Ok(Value::Int(x)),
        None => Err(format!("String cannot be parsed to integer ({})", s)),
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

/// Parse an XMLRPC response
pub fn parse_response(response_str: &str) -> Result<Response, String> {
    // Technically we should remove the http header first, but the xml
    // parser will actually ignore it and work anyway. Unless there is
    // a "<" in it. Then this will totally fail.
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

