use std::io::prelude::*;
use std::fs::File;

extern crate redis;
use redis::{Client, Commands};

fn main() {
    let mut f = File::open("examples/not_kitty.png").expect("cannot open the file");
    let mut buffer = Vec::new();
    f.read_to_end(&mut buffer).expect("cannot read to end");
    let mut fw = File::create("examples/c_not_kitty.png").expect("cannot create the file");
    // fw.write_all(&buffer[..]).expect("cannot write the file");
    let client = Client::open("redis://127.0.0.1/").unwrap();
    let conn = client.get_connection().unwrap();
    let _ : () = conn.set("pic_not_kitty", buffer).unwrap();
    // "get" uses "fromRedisResult", which requires the param to have the "sized" trait, so need to explicitly claim buffer1 as Vec<u8> here
    let buffer1:Vec<u8> = conn.get("pic_not_kitty").unwrap();
    fw.write_all(&buffer1[..]).expect("cannot write the file");
}