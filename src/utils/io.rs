use std::io::{Read, BufReader};
use std::fs::File;

pub fn load_as_bytes(path: &str) -> Option<Vec<u8>> {
    match File::open(path) {
        Ok(file) => {
            let mut rdr = BufReader::new(file);
            let mut buf: Vec<u8> = Vec::new();
            match rdr.read_to_end(&mut buf) {
                Ok(_) => Some(buf),
                _ => None
            }
        }
        _ => None
    }
}
