extern crate sha1;

use std::io;

use sha1::sha1;
use sha1::hex;


fn main() {

    let input = io::stdin().read_line()
                             .ok()
                             .expect("Failed to read line.");

    
    println!("{}", hex(sha1(input.as_bytes()))) ;
}
