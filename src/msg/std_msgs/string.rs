use std::string::String as std_String;
use std::old_io::Reader;

pub struct String {
    pub data: std_String,
}

impl String {
    pub fn from_stream<R: Reader>(stream : &mut R) -> Result<String, std_String> {
        let string_size = match stream.read_le_u32() {
            Ok(string_size) => string_size,
            Err(_) => return Err("Failed to read string length".to_string()),
        };

        // Read the string data
        let string_data_bytes = match stream.read_exact(string_size as usize) {
            Ok(string_data_bytes) => string_data_bytes,
            Err(_) => return Err("Failed to read string data field".to_string()),
        };

        let data = match std_String::from_utf8(string_data_bytes) {
            Ok(data) => data,
            Err(_) => return Err("Failed to interpret string data as utf8 string".to_string()),
        };

        Ok(String {data: data})
    }
}

