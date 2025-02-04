use std::fs::File;
use std::io::prelude::*;

use hex::FromHex;

use nom::HexDisplay;
use redis::{Client, Commands};

#[allow(dead_code)]
fn t_cpu_info() {
    println!(
        "available_parallelism: {0}",
        std::thread::available_parallelism().unwrap().get()
    );
    let cpus = num_cpus::get();
    println!(
        "logical_cpus={}, physical_cpus={}",
        num_cpus::get(),
        num_cpus::get_physical()
    );
}

fn t_redis() {
    let mut f = File::open("res/not_kitty.png").expect("cannot open the file");
    let mut buffer = Vec::new();
    f.read_to_end(&mut buffer).expect("cannot read to end");
    let mut fw = File::create("res/c_not_kitty.png").expect("cannot create the file");
    let client = Client::open("redis://127.0.0.1/").unwrap();
    let mut conn = client.get_connection().unwrap();
    let _: () = conn.set("pic_not_kitty", buffer).unwrap();
    // "get" uses "fromRedisResult", which requires the param to have the "sized" trait, so need to explicitly claim buffer1 as Vec<u8> here
    let new_buffer: Vec<u8> = conn.get("pic_not_kitty").unwrap();
    fw.write_all(&new_buffer[..])
        .expect("cannot write the file");
}

fn t_term() {
    let mut t = term::stdout().unwrap();
    t.fg(term::color::GREEN).unwrap();
    write!(t, "hello ").unwrap();
    t.bg(term::color::RED).unwrap();
    writeln!(t, "world!").unwrap();
    t.reset().unwrap();
}

fn t_hex() {
    let hex_string = b"666f6f626172";
    let s = Vec::from_hex(&hex_string).unwrap();
    println!("{:?}", s);
    println!("{:?}", hex::encode(s));
}

fn main() {
    // t_term();
    // t_cpu_info();
    // t_redis();
    t_hex();
}
