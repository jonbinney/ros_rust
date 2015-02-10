extern crate ros_rust;

use std::mem;
use std::os;
use std::old_io::TcpStream;

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

    println!("Reading data");
    loop {
        match stream.read_byte() {
            Ok(b) => println!("{}", b.to_string()),
            Err(_) => panic!("Read failed!"),
        }
    }
}


