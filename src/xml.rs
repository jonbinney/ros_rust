//! Module for parsing and serializing XML documents.
//!
//! Only implements the subset of the XML specification needed to
//! parse XMLRPC requests and responses - does not implement attributes,
//! for example.
//!
//! The names of the regular expressions for the tokens are chosen to try
//! and match the XML spec: http://www.w3.org/TR/REC-xml/

use regex;

/// An XML element. An element in an XML document is defined by a start and
/// end tag, and may have text or other elements inside of it. There is also
/// an implicit "root" element which includes all other elements.
#[deriving(Show, PartialEq, Clone)]
pub struct Element {
    name: String,
    text: String,
    children: Vec<Element>,
}

/// Convenience function to avoid calling .to_string() for name and text member
fn make_element(name: &str, text: &str, children: Vec<Element>) -> Element {
    Element {name: name.to_string(), text: text.to_string(), children: children}
}

enum Token {
    PI, // Processing instruction, e.g. <?xml version="1.0"?>.
    STag(String), // Start tag
    ETag(String), // End tag
    Text(String), // Text
}

pub fn parse_xml(input_str: &str) -> Result<Element, String> {
    let mut remaining_str = input_str;

    // Ignore any preceeding (non-tag) text
    remaining_str = match get_text_token(remaining_str) {
        None => remaining_str,
        Some((Token::Text(_), new_remaining_str)) => new_remaining_str,
        Some((_, _)) => panic!("Bad token type returned"),
    };

    // Remove any initial <?xml version="foo"?> tag
    remaining_str = match get_pi_token(remaining_str) {
        None => remaining_str,
        Some((Token::PI, new_remaining_str)) => new_remaining_str,
        Some((_, _)) => panic!("Bad token type returned"),
    };

    // Parse the main element (assumes there is exactly one)
    let element = match parse_element(remaining_str) {
        Ok((el, _)) => el,
        Err(err) => return Err(err),
    };
    Ok(element)
}

fn parse_element(input_str: &str) -> Result<(Element, &str), String> {
    let mut remaining_str = input_str;
    let mut element = make_element("", "", vec![]);

    // Ignore any preceeding (non-tag) text
    remaining_str = match get_text_token(remaining_str) {
        None => remaining_str,
        Some((Token::Text(_), new_remaining_str)) => new_remaining_str,
        Some((_, _)) => panic!("Bad token type returned"),
    };

    // Parse starting tag
    remaining_str = match get_stag_token(remaining_str) {
        None => return Err("No starting tag".to_string()),
        Some((Token::STag(tag_name), new_remaining_str)) => {
            element.name = tag_name;
            new_remaining_str
        },
        Some((_, _)) => panic!("Bad token type returned"),
    };

    // Parse body text of this element
    remaining_str = match get_text_token(remaining_str) {
        None => remaining_str,
        Some((Token::Text(text), new_remaining_str)) => {
            element.text = text;
            new_remaining_str
        },
        Some((_, _)) => panic!("Bad token type returned"),
    };

    // Parse any child elements
    loop {
        match parse_element(remaining_str) {
            Ok((child_element, new_remaining_str)) => {
                element.children.push(child_element);
                remaining_str = new_remaining_str;
            },
            Err(_) => {break;},
        }
    }

    // Ignore text after children
    remaining_str = match get_text_token(remaining_str) {
        None => remaining_str,
        Some((Token::Text(_), new_remaining_str)) => new_remaining_str,
        Some((_, _)) => panic!("Bad token type returned"),
    };

    // Parse ending tag
    remaining_str = match get_etag_token(remaining_str) {
        None => return Err("No starting tag".to_string()),
        Some((Token::ETag(end_tag_name), new_remaining_str)) =>
            if end_tag_name == element.name {
                new_remaining_str
            }
            else {
                return Err(format!("Start tag {} does not match end tag {}",
                    element.name, end_tag_name));
            },
        Some((_, _)) => panic!("Bad token type returned"),
    };

    Ok((element, remaining_str))
}

pub fn serialize_xml(element: &Element) -> String {
    "<?xml version=\"1.0\"?>\n".to_string() + serialize_element(element)
}

fn serialize_element(element: &Element) -> String {
    let mut result = "".to_string();

    result = result + format!("<{}>{}", element.name, element.text);
    for child_element in element.children.iter() {
        result = result + serialize_element(child_element);
    }
    result = result + format!("</{}>", element.name);

    result
}

fn get_pi_token(input_str: &str) -> Option<(Token, &str)> {
    let pi_re = regex!("<[?][^>]*[?]>");
    match pi_re.captures(input_str) {
        None => None,
        Some(caps) => Some((Token::PI, get_remaining_string(&caps, input_str))),
    }
}

fn get_stag_token(input_str: &str) -> Option<(Token, &str)> {
    let stag_re = regex!("^<([:alnum:]+)[:space:]*>");
    match stag_re.captures(input_str) {
        None => None,
        Some(caps) => Some((Token::STag(caps.at(1).to_string()),
                    get_remaining_string(&caps, input_str))),
    }
}

fn get_etag_token(input_str: &str) -> Option<(Token, &str)> {
    let etag_re = regex!("^</([:alnum:]+)[:space:]*>");
    match etag_re.captures(input_str) {
        None => None,
        Some(caps) => Some((Token::ETag(caps.at(1).to_string()),
            get_remaining_string(&caps, input_str))),
    }
}

fn get_text_token(input_str: &str) -> Option<(Token, &str)> {
    let text_re = regex!("(^[^<]+)");
    match text_re.captures(input_str) {
        None => None,
        Some(caps) => Some((Token::Text(caps.at(1).to_string()),
            get_remaining_string(&caps, input_str))),
    }
}

fn get_remaining_string<'a>(caps: &regex::Captures, input_str: &'a str) -> &'a str {
    match caps.pos(0) {
        None => panic!("Unexpected empty capture group"),
        Some((_, end_i)) => input_str.slice_from(end_i)
    }
}

#[test]
fn test_get_pi_token() {
    // Should match
    match get_pi_token("<? foo ?> asdf") {
        None => return assert!(false, "Failed to match"),
        Some((Token::PI, rem)) => assert_eq!(rem, " asdf"),
        _ => assert!(false, "Bad match"),
    };

    // Standard XMLDecl
    match get_pi_token("<?xml version=\"1.0\"?><sdf>") {
        None => return assert!(false, "Failed to match"),
        Some((Token::PI, rem)) => assert_eq!(rem, "<sdf>"),
        _ => assert!(false, "Bad match"),
    };

    // Shouldn't match a normal tag
    match get_pi_token("<foo>") {
        Some(_) => return assert!(false, "Incorrect match"),
        _ => (),
    };
}

#[test]
fn test_get_stag_token() {
    // Should match
    match get_stag_token("<foo> asdf") {
        None => return assert!(false, "Failed to match"),
        Some((Token::STag(name), rem)) => assert_eq!((name.as_slice(), rem), ("foo", " asdf")),
        _ => assert!(false, "Bad match"),
    };

    // Should match even with a space after the name
    match get_stag_token("<foo > asdf") {
        None => return assert!(false, "Failed to match"),
        Some((Token::STag(name), rem)) => assert_eq!((name.as_slice(), rem), ("foo", " asdf")),
        _ => assert!(false, "Bad match"),
    };

    // Shouldn't match an end tag
    match get_stag_token("<foo/>") {
        Some(_) => return assert!(false, "Incorrect match"),
        _ => (),
    };
}

#[test]
fn test_get_etag_token() {
    // Should match
    match get_etag_token("</foo> asdf") {
        None => return assert!(false, "Failed to match"),
        Some((Token::ETag(name), rem)) => assert_eq!((name.as_slice(), rem), ("foo", " asdf")),
        _ => assert!(false, "Bad match"),
    };

    // Should match even with a space after the name
    match get_etag_token("</foo > asdf") {
        None => return assert!(false, "Failed to match"),
        Some((Token::ETag(name), rem)) => assert_eq!((name.as_slice(), rem), ("foo", " asdf")),
        _ => assert!(false, "Bad match"),
    };

    // Shouldn't match a start tag
    match get_etag_token("<foo>") {
        Some(_) => return assert!(false, "Incorrect match"),
        _ => (),
    };
}

#[test]
fn test_get_text_token() {
    // Should match
    match get_text_token(" asdf asdf <") {
        None => return assert!(false, "Failed to match"),
        Some((Token::Text(text), rem)) => assert_eq!((text.as_slice(), rem), (" asdf asdf ", "<")),
        _ => assert!(false, "Bad match"),
    };

    // Shouldn't match a start tag
    match get_text_token("<foo>") {
        Some(_) => return assert!(false, "Incorrect match"),
        _ => (),
    };
}

#[test]
fn test_parse_xml() {
    // Simplest possible test
    match parse_xml("<foo></foo>") {
        Err(_) => assert!(false, "Failed to parse"),
        Ok(element) => assert_eq!(
            element,
            make_element("foo", "", vec![])
        ),
    }

    // Should be able to parse an xmlrpc response
    match parse_xml(
        "\
        <?xml version=\"1.0\"?>\n\
        <methodResponse>\n\
          <params>\n\
            <param>\n\
              <value><string>some_string_param</string></value>\n\
            </param>\n\
          </params>\n\
        </methodResponse>\n\
        ") {
        Err(_) => assert!(false, "Failed to parse"),
        Ok(element) => assert_eq!(
            element,
            make_element("methodResponse", "\n", vec![
                make_element("params", "\n", vec![
                    make_element("param", "\n", vec![
                        make_element("value", "", vec![
                            make_element("string", "some_string_param", vec![])
                        ])
                    ])
                ])
            ])
        ),
    }
}

#[test]
fn test_serialize_xml() {
    let element = make_element("methodCall", "\n", vec![
        make_element("methodName", "foo", vec![]),
        make_element("params", "\n", vec![
            make_element("param", "\n", vec![
                make_element("value", "", vec![
                    make_element("string", "some_string_param", vec![])
                ])
            ]),
            make_element("param", "\n", vec![
                make_element("value", "", vec![
                    make_element("int", "some_int_param", vec![])
                ])
            ])
        ])
    ]);

    // Serialize and de-serialize to test full loop.
    let serialized_xml = serialize_xml(&element);
    match parse_xml(serialized_xml.as_slice()) {
        Err(_) => panic!("Failed to parse serialized xml"),
        Ok(parsed_element) => assert_eq!(parsed_element, element),
    };
}

