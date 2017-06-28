extern crate shmem;

extern crate nix;
use std::process::Command;
use std::env;

use shmem::array;
use nix::sys::signal::*;
use nix::unistd::*;

// main process does the read
// child process does the write
//

const ENV_VAR: &str = "GOOD";

pub fn write() {

    let file_name = env::var(ENV_VAR).expect("ENV_VAR not set");
    println!("filename={}", file_name);
    let mut pa: array::Owned<u8> = shmem::array::create::<u8, _>(file_name, SIZE).expect("error");

    for (i, item) in pa.iter_mut().enumerate() {
        println!("i={}", i);
        *item = i as u8;
    }

}

const SIZE: usize = 40;

pub fn main() {

    let file_name = "xxxooo";

    env::set_var(ENV_VAR, file_name);


    match fork().expect("fork failed") {
        ForkResult::Parent { child } => {
            sleep(1);
            let mut pa: array::Owned<u8> =
                shmem::array::open::<u8, _>(file_name).expect("error");
            println!("{:?}", &*pa);

        }
        ForkResult::Child => {
            write();
        }
    }




}
