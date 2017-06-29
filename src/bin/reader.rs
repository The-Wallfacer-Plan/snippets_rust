#![allow(unused_variables, dead_code, unused_imports)]
extern crate shmem;
extern crate nix;

use nix::sys::signal::*;
use nix::unistd::*;
use nix::sys::wait::*;

fn main() {
    #[allow(blacklisted_name)]
    let bar = shmem::array::open::<u8, _>("shmem-rust-array-xx").unwrap();
    println!("{:?}", &*bar);
}
