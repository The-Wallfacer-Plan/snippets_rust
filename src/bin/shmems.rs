#![allow(non_snake_case,unused_variables,dead_code, unused_imports)]
extern crate shmem;

extern crate nix;
use std::process::Command;
use std::env;

use shmem::array;
use nix::sys::signal::*;
use nix::unistd::*;
use nix::sys::wait::*;

const ENV_VAR: &str = "GOOD";

pub fn write() {

    let file_name = env::var(ENV_VAR).expect("ENV_VAR not set");
    println!("filename={}", file_name);
    let mut pa: array::Owned<u8> =
        shmem::array::open::<u8, _>(file_name).expect("writer open shmem failed");

    for (i, item) in pa.iter_mut().enumerate() {
        println!("i={}", i);
        *item = (SIZE - i) as u8;
    }

}

const SIZE: usize = 8;

pub fn main() {

    let file_name = "elojj";

    shmem::array::create::<u8, _>(file_name, SIZE).expect("reader create failed");
    env::set_var(ENV_VAR, file_name);

    match fork().expect("fork failed") {
        ForkResult::Parent { child } => {
            sleep(1);
            let wait_status = waitpid(child, None);
            match wait_status {
                Ok(WaitStatus::Exited(_, _)) => {
                    let pa: array::Owned<u8> =
                        shmem::array::open::<u8, _>(file_name).expect("reader open shmem failed");
                    println!("{:?}", &*pa);
                }
                Ok(_) => panic!("Child still alive, should never happen"),
                Err(_) => panic!("Error: waitpid Failed"),
            }
        }
        ForkResult::Child => {
            write();
        }
    }

}
