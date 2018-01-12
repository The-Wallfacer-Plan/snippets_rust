extern crate hex;

use hex::*;

fn main() {
    let s = Vec::from_hex(b"666f6f626172").unwrap();
    println!("{:?}", s);
    let mut ss = String::new();
    "foobar".write_hex(&mut ss).unwrap();
    println!("{:?}", ss);
}
