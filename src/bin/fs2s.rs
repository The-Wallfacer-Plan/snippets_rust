extern crate fs2;

use fs2::*;

fn main() {
    use std::env;
    use std::fs::File;
    let home_dir = env::home_dir().unwrap();
    let path = home_dir.join(".vimrc");
    let f = File::open(&path).unwrap();
    println!("allocated_size: {}", f.allocated_size().unwrap());
    println!("free_space: {}", free_space(&path).unwrap());
}
