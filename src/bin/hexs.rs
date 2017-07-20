extern crate hex;

use hex::*;

fn main() {
    let s = Vec::from_hex(b"666f6f626172").unwrap();
    println!("{:?}", s);
    let ss = "foobar".to_hex();
    println!("{:?}", ss);
}
