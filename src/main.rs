extern crate sha1;

use std::io;

use sha1::sha1;
use sha1::print_hex;


fn main() {

    let input = io::stdin().read_line()
                             .ok()
                             .expect("Failed to read line.");

    
    print_hex(&sha1(input.trim().as_bytes()));

    return;
}
