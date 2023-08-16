extern crate hex;

use hex::FromHex;

fn fixed_xor(buffer1: &[u8], buffer2: &[u8]) -> Vec<u8> {
    buffer1.iter().zip(buffer2.iter()).map(|(b1, b2)| b1 ^ b2).collect()
}

fn main() {
    let hex_string1 = "1c0111001f010100061a024b53535009181c";
    let hex_string2 = "686974207468652062756c6c277320657965";

    let byte_array1 = Vec::from_hex(&hex_string1).expect("Invalid hex input");
    let byte_array2 = Vec::from_hex(&hex_string2).expect("Invalid hex input");

    let xor = fixed_xor(&byte_array1, &byte_array2);

    let xor_hex_string: String = xor.iter().map(|b| format!("{:02x}", b)).collect();

    println!("xor_hex_string : {}", xor_hex_string);
}