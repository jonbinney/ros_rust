//! Module for parsing XML documents.
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
pub struct Element {
    name: String,
    children: Vec<Element>,
}

enum Token {
    PI, // Processing instruction, e.g. <?xml version="1.0"?>.
    STag(String), // Start tag
    ETag(String), // End tag
    Text(String), // Text
}

pub fn parse_recursive(input_str: &str) -> Result<Element, String> {
    let remaining_str = input_str;
    let mut root = Element {name: "root".to_string(), children: vec![]};

    let mut open_elements: Vec<&Element> = vec![];
    open_elements.push(&root);
    loop {
        let (tok, remaining_str) = match get_token(remaining_str) {
            Some((Token, new_remaining_str)) => if new_remaining_str.len() < remaining_str.len() {
                (Token, new_remaining_str)} else {panic!("Caught in parsing loop")},
            None => break,
        };

        match tok {
            Token::STag(new_tag_name) => {
                let num_elements = open_elements.len();
                let &mut parent_element = match open_elements.get_mut(num_elements-1) {
                    None => return Err("Root element has gone missing".to_string()),
                    Some(&mut parent_element) => parent_element,
                };
                parent_element.children.push(Element {name: new_tag_name, children: vec![]});

                // Get a reference to the child we just created, and add it to the vector
                // of open elements. (This seems like an ugly way to do this...)
                let num_children = parent_element.children.len();
                open_elements.push(match parent_element.children.get_mut(num_children-1) {
                    None => panic!("Child element we just created disappeared..."),
                    Some(mut new_element) => new_element,
                });
            },

            // Need to add handlers for other cases
            _ => {},
        }

    }

    // In a properly formed document, only the root element should be left open
    match open_elements.len() {
        0 => Err("Root element closed explicitly".to_string()),
        1 => match open_elements.pop() {
            Some(x) => Ok(*x),
            None => panic!("Unexpected empty vector"),
        },
        l => Err(format!("{} unclosed elements", l)),
    }
}

/// Parse the next token from the given string.
///
/// Returns the token and the remaining unparsed part of the string.
fn get_token(input_str: &str) -> Option<(Token, &str)> {
    match get_pi_token(input_str) {
        None => (),
        x => return x,
    };

    match get_stag_token(input_str) {
        None => (),
        x => return x,
    };

    match get_etag_token(input_str) {
        None => (),
        x => return x,
    };

    get_text_token(input_str)
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

