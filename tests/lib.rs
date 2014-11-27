extern crate sha1;
use sha1::sha1;
use sha1::hex;

#[test]
fn sha1_empty_string() {
    // da39a3ee5e6b4b0d3255bfef95601890afd80709
    let expected = "da39a3ee5e6b4b0d3255bfef95601890afd80709";

    assert_eq!(expected, hex(&sha1("".as_bytes())).as_slice());
}
