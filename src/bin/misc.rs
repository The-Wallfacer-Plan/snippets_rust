extern crate hex;

extern crate num_cpus;
extern crate redis;
extern crate term;

use std::fs::File;
use std::io::prelude::*;

use hex::FromHex;

use redis::{Client, Commands};

#[allow(dead_code)]
fn cpu_info() {
    let cpus = num_cpus::get();
    println!("cpus={}", cpus);
}

#[allow(dead_code)]
fn t_redis() {
    let mut f = File::open("res/not_kitty.png").expect("cannot open the file");
    let mut buffer = Vec::new();
    f.read_to_end(&mut buffer).expect("cannot read to end");
    let mut fw = File::create("res/c_not_kitty.png").expect("cannot create the file");
    // fw.write_all(&buffer[..]).expect("cannot write the file");
    let client = Client::open("redis://127.0.0.1/").unwrap();
    let mut conn = client.get_connection().unwrap();
    let _: () = conn.set("pic_not_kitty", buffer).unwrap();
    // "get" uses "fromRedisResult", which requires the param to have the "sized" trait, so need to explicitly claim buffer1 as Vec<u8> here
    let buffer1: Vec<u8> = conn.get("pic_not_kitty").unwrap();
    fw.write_all(&buffer1[..]).expect("cannot write the file");
}

#[allow(dead_code)]
fn t_term() {
    let mut t = term::stdout().unwrap();
    t.fg(term::color::GREEN).unwrap();
    write!(t, "hello ").unwrap();
    t.bg(term::color::RED).unwrap();
    writeln!(t, "world!").unwrap();
    t.reset().unwrap();
}

#[allow(dead_code)]
fn t_hex() {
    let s = Vec::from_hex(b"666f6f626172").unwrap();
    println!("{:?}", s);
    let _ss = String::new();
}

fn main() {
    t_term();
}
