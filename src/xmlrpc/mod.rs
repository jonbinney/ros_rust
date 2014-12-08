pub mod parser;
pub mod client;

#[deriving(Show, PartialEq)]
pub enum Value {
    Int (int),
    Boolean (bool),
    String (String),
    Double (f64),
    // Currently not handling dateTime.iso8601 base64 or struct types
}

#[deriving(Show, PartialEq)]
pub enum Response {
    Success {param: Value},
    Fault {fault_code: int, fault_string: String},
}

#[deriving(Show, PartialEq)]
pub struct Request {
    pub method_name: String,
    pub params: Vec<Value>,
}

