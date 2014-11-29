extern crate crypto;
extern crate serialize;


use std::io;
use crypto::hash::{Hasher, sha1};
use serialize::hex::ToHex;

fn main() {

    let input = io::stdin().read_line()
                             .ok()
                             .expect("Failed to read line.");

    let mut m = sha1::SHA1::new();
    m.update(input.trim().as_bytes());

    let hh = m.digest().as_slice().to_hex();
    println!("{}", hh);
    

    return;
}
