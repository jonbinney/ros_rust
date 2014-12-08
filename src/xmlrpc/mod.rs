#[deriving(Show, PartialEq)]
pub enum Value {
    int (int),
    boolean (bool),
    string (String),
    double (f64),
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

pub mod parser;