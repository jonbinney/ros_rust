extern crate ros_rust;

use std::mem;
use std::os;
use std::old_io::TcpStream;
use std::old_io::MemReader;
use std::collections::HashMap;

use ros_rust::msg::std_msgs;

/// Read a TCPROS connection header into a HashMap of key and value strings
fn read_connection_header<R: Reader>(stream: &mut R) -> Result<HashMap<String, String>, String> {
    let header_length = match stream.read_le_u32() {
        Ok(header_length) => header_length,
        Err(_) => return Err("Failed to read connection header length".to_string()),
    };

    // Read the connection header fields
    let mut header_bytes_read = 0u32;
    let mut fields: HashMap<String, String> = HashMap::new();
    while header_bytes_read < header_length {
        // Read the length of this field
        let field_length = match stream.read_le_u32() {
            Ok(field_length) => field_length,
            Err(_) => return Err("Failed to read connection header length".to_string()),
        };
        header_bytes_read += 4;

        // Read the field itself
        let field_bytes = match stream.read_exact(field_length as usize) {
            Ok(field_bytes) => field_bytes,
            Err(_) => return Err("Failed to read connection header field".to_string()),
        };

        let field_string = match String::from_utf8(field_bytes) {
            Ok(s) => s,
            Err(_) => return Err("Failed to interpret connection header field as string".to_string()),
        };

        println!("{:?}", field_string);

        header_bytes_read += field_length;
    }
    Ok(fields)
}

#[allow(dead_code)]
fn main() {
    let args: Vec<String> = os::args();
    let tcpros_server_address = args[1].clone();

    println!("Connecting to {}", tcpros_server_address);

    // Connect to the server
    let mut stream = match TcpStream::connect(tcpros_server_address.as_slice()) {
        Ok(x) => x,
        Err(_) => panic!("Unable to connect to tcpros server".to_string()),
    };

    let fields = [
        "message_definition=string data\n\n",
        "callerid=/foo_callerid",
        "topic=/foo",
        "md5sum=992ce8a1687cec8c8bd883ec73ca41d1",
        "type=std_msgs/String"];

    // Create the request
    let mut connection_header = Vec::new();

    for field in fields.iter() {
        println!("{}, {}", field.len(), field);
        connection_header.write_le_u32(field.len() as u32);
        connection_header.write_str(field);
    }
    println!("{:?}", connection_header.as_slice());

    // Send connection header length to server
    match stream.write_le_u32(connection_header.len() as u32) {
        Ok(_) => (),
        Err(_) => panic!("Unable to send data to server".to_string()),
    };

    // Send connection header to server
    println!("Sending header");
    match stream.write_all(&connection_header.as_slice()) {
        Ok(_) => (),
        Err(_) => panic!("Unable to send data to server".to_string()),
    };

    // Read the connection header that the server sends
    let connection_header_fields = match read_connection_header(&mut stream) {
        Ok(fields) => fields,
        Err(err) => panic!(err),
    };

    println!("Reading data");
    loop {
        match std_msgs::String::from_stream(&mut stream) {
            Ok(s) => println!("{}", s.data),
            Err(_) => panic!("Read failed!"),
        }
    }
}


