

#![feature(ascii_char)]
#![feature(const_option)]

use std::{fs::File, io::Write};

use hex_data::DATA;
use parser::read_file;

mod parser;
mod hex_data;

fn main() {

    let (file_str,len) = read_file();

    // Open the lib.rs file in write mode
    let file_path = "../deploy/src/hex_data.rs";
    let mut file_handle = match File::create(file_path) {
        Ok(file) => file,
        Err(e) => {
            eprintln!("Failed to create file '{}': {}", file_path, e);
            return;
        }
    };
    file_handle.write(format!("pub const FILE_SIZE: i32 = {};\n", len).as_bytes()).unwrap();
    file_handle.write(format!("pub const DATA: [u8; {}] = \n ", len).as_bytes()).unwrap();
    file_handle.write(file_str.as_bytes()).unwrap();
    file_handle.write("; \n".as_bytes()).unwrap();
    // let mut i= 0;
    // println!("hex_data[0]: {}", DATA[7] as u32);


}
