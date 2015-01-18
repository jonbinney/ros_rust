use xmlrpc::common::Value;

macro_rules! validate_xmlrpc_value_recursive {
    ( $v:ident, ( $($T:tt),* ) ) => (
        match $v {
            &Value::Array(ref a) => {
                // Create a tuple whose elements are recursively filled in
                let mut ii: isize = -1;
                ( $(
                    if ii >= a.len() as isize {
                        return Err("Not enough elements in array".to_string());
                    } else {
                        ii += 1;
                        let ref e = a[ii as usize];
                        validate_xmlrpc_value_recursive!(e, $T)
                    }
                ),* )
            },
            _ => return Err("Expected array, found something else".to_string()),
        }
    );
    ( $v:ident, i32 ) => ( match $v {
        &Value::Int(ref x) => x.clone(),
        _ => return Err("Expected int; found something else".to_string()),
    } );
    ( $v:ident, String ) => ( match $v {
        &Value::String(ref x) => x.clone(),
        _ => return Err("Expected string; found something else".to_string()),
    } );
    ( $v:ident, f64 ) => ( match $v {
        &Value::Double(ref x) => x.clone(),
        _ => return Err("Expected double; found something else".to_string()),
    } );
}

/// Just calls the macro that does the real parsing, and wraps the result in Ok()
/// since we expect a Result<...> at the top level.
macro_rules! validate_xmlrpc_value_top_level {
    ( $v:ident, $T:tt ) => (
        Ok(validate_xmlrpc_value_recursive!($v, $T))
    );
}

/// Validate an xmlrpc value using a type specified as nested tuples.
macro_rules! validate_xmlrpc_value {
    ( $v:ident, $T:tt ) => (
        {
            // From here on down, we pass references
            let v_ref: &Value = & $v;

            // Wrap the validation call in a closure so that we can return errors
            // when we want to stop validating early.
            (|&:| {validate_xmlrpc_value_top_level!(v_ref, $T)})()
        }
    );
}

/// Validate an xmlrpc response using a type specified as nested tuples.
macro_rules! validate_xmlrpc_response {
    ( $response:ident, $T:tt ) => (
        match $response {
            Response::Fault {fault_code, fault_string} => Err(format!(
                "Fault: {} ({})", fault_string, fault_code)),
            Response::Success {param} => validate_xmlrpc_value!(param, $T),
        }
    );
}

/// Validate an xmlrpc request using a type specified as nested tuples.
macro_rules! validate_xmlrpc_request {
    ( $request:ident, $T:tt ) => (
        validate_xmlrpc_value!(xmlrpc::Value::Array($request.params), $T),
    );
}

#[cfg(test)]
mod tests {
    use xmlrpc::common::{Value, Request, Response};

    #[test]
    fn test_validate_xmlrpc_value() {
        let v = Value::Array(vec![
            Value::Int(1i32),
            Value::Int(3i32)
            ]);

        let x = validate_xmlrpc_value!(v, (i32, i32));
        assert_eq!(x, Ok((1i32, 3i32)));
    }

    #[test]
    fn test_validate_xmlrpc_value_incorrect_types() {
        let v = Value::Array(vec![
            Value::Int(1i32),
            Value::Int(3i32)
            ]);

        let x = validate_xmlrpc_value!(v, (i32, String));
        match x {
            Ok(_) => panic!("Validation succeeded with incorrect types"),
            Err(_) => {},
        };
    }

    #[test]
    fn test_validate_xmlrpc_response_success() {
        let v = Response::Success {
            param: Value::Array(vec![
                Value::String("foo".to_string()),
                Value::Array(vec![
                    Value::Int(3i32),
                    Value::Double(3.14) ])]) };

        let x = validate_xmlrpc_response!(v, (String, (i32, f64)));
        match x {
            Ok((x0, (x1, x2))) => assert_eq!((x0, (x1, x2)), ("foo".to_string(), (3i32, 3.14))),
            Err(err) => panic!(err),
        }
    }

    #[test]
    fn test_validate_xmlrpc_response_fault() {
        let v = Response::Fault {fault_code: 22, fault_string: "some_fault".to_string()};

        let x  = validate_xmlrpc_response!(v, (String, (i32, f64)));
        match x {
            Ok(_) => panic!("Validation succeeded when it should have found fault"),
            Err(_) => {},
        }
    }
}
