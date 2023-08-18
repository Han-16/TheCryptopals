extern crate hex;
extern crate base64;

use hex::FromHex;
use base64::encode;

fn main() {
    let hex_string = String::from("49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d");
    let byte_array = Vec::from_hex(hex_string).expect("Invalid hex input");
    let base64_encoded = encode(&byte_array);

    println!("Base64 encoded: {}", base64_encoded);
}