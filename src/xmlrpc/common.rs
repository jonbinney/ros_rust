#[derive(Show, PartialEq)]
pub enum Value {
    Int (i32),
    Boolean (bool),
    String (String),
    Double (f64),
    Array (Vec<Value>),
    // Not handling dateTime.iso8601 base64 or struct types
}

#[derive(Show, PartialEq)]
pub enum Response {
    Success {param: Value},
    Fault {fault_code: i32, fault_string: String},
}

#[derive(Show, PartialEq)]
pub struct Request {
    pub method_name: String,
    pub params: Vec<Value>,
}

