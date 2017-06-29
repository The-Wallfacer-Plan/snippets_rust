extern crate shmem;
extern crate nix;

use std::thread;
use nix::sys::signal::*;
use nix::unistd::*;
use nix::sys::wait::*;


fn main() {

    #[allow(blacklisted_name)]
    let mut bar = shmem::array::create::<u8, _>("shmem-rust-array-xx", 10).unwrap();
    for (i, item) in bar.iter_mut().enumerate() {
        *item = (10-i) as u8;
    }
    sleep(10);

}
