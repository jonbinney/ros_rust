use xmlrpc::common::Value;

macro_rules! parse_xmlrpc_value_recursive {
    ( $v:ident, ( $($T:tt),* ) ) => (
        match $v {
            &Value::Array(ref a) => {
                // Create a tuple whose elements are recursively filled in
                let mut ii: isize = -1;
                ( $(
                    if ii >= a.len() as isize {
                        return Err("Not enough elements in array");
                    } else {
                        ii += 1;
                        let ref e = a[ii as usize];
                        parse_xmlrpc_value_recursive!(e, $T)
                    }
                ),* )
            },
            _ => return Err("Expected array, found something else"),
        }
    );
    ( $v:ident, i32 ) => ( match $v {
        &Value::Int(ref x) => x.clone(),
        _ => return Err("Expected int; found something else"),
    } );
    ( $v:ident, String ) => ( match $v {
        &Value::String(ref x) => x.clone(),
        _ => return Err("Expected string; found something else"),
    } );
}

/// Just calls the macro that does the real parsing, and wraps the result in Ok()
/// since we expect a Result<...> at the top level.
macro_rules! parse_xmlrpc_value_top_level {
    ( $v:ident, $T:tt ) => (
        Ok(parse_xmlrpc_value_recursive!($v, $T))
    );
}

macro_rules! parse_xmlrpc_value {
    ( $v:ident, $T:tt ) => (
        {
            // From here on down, we pass references
            let v_ref: &Value = & $v;
            (|&:| {parse_xmlrpc_value_top_level!(v_ref, $T)})()
        }
    );
}

#[cfg(test)]
mod tests {
    use xmlrpc::common::Value;

    #[test]
    fn test_parse_value() {
        let v = Value::Array(vec![
            Value::Int(1i32),
            Value::Int(3i32)
            ]);

        let x = parse_xmlrpc_value!(v, (i32, i32));
        assert_eq!(x, Ok((1i32, 3i32)));
    }
}
