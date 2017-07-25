#![allow(non_snake_case,unused_variables,dead_code, unused_imports)]
extern crate libc;
extern crate errno;
extern crate nix;

use std::process::Command;
use std::env;
use std::ffi::CString;
use std::ffi::CStr;
use std::ptr;

use nix::sys::signal::*;
use nix::unistd::*;
use nix::sys::wait::*;

const ENV_VAR: &str = "GOOD";
const SIZE: usize = 8;

unsafe fn str_to_cchar(s: &str) -> CString {
    CString::new(s).unwrap()
}

fn get_shm_id(key: libc::key_t) -> i32 {
    unsafe {
        // 0o means octal number
        let shm_id = libc::shmget(key, SIZE, libc::IPC_CREAT | 0o666);
        if shm_id < 0 {
            panic!("ALERT: shm_id={}, errno={}", shm_id, errno::errno());
        }
        shm_id
    }
}

unsafe fn attach_shm(shm_id: i32) -> *mut libc::c_char {
    let pa_v = libc::shmat(shm_id, ptr::null(), 0);
    if (pa_v as isize) == -1 {
        panic!("Attach shm failed: {}, shm_id={}", errno::errno(), shm_id);
    }
    pa_v as *mut libc::c_char
}

fn remove_shm(shm_id: i32) {
    unsafe {
        libc::shmctl(shm_id, libc::IPC_RMID, ptr::null_mut());
    }
}


pub fn main() {
    unsafe {
        let shm_id = get_shm_id(0 as libc::key_t);

        let mut count: i32 = 0;

        for x in 0..3 {
            match fork().expect("fork failed") {
                ForkResult::Parent { child } => {
                    sleep(1);
                    let wait_status = waitpid(child, None);
                    match wait_status {
                        Ok(WaitStatus::Exited(_, _)) => {
                            let slice = [0u8; SIZE];
                            let pa: *mut libc::c_char = attach_shm(shm_id);
                            // let slice = CStr::from_ptr(pa);
                            let pa_s = ::std::slice::from_raw_parts(pa, SIZE);
                            slice.copy_from_slice(pa_s);
                            println!("{:?}", slice);
                        }
                        Ok(_) => panic!("Child still alive, should never happen"),
                        Err(_) => panic!("Error: waitpid Failed"),
                    }
                }
                ForkResult::Child => {
                    let pa: *mut libc::c_char = attach_shm(shm_id);
                    count += 1;
                    let cs: CString = str_to_cchar(&format!("child: {}", count));
                    let ss: *const libc::c_char = cs.as_ptr();
                    libc::strcpy(pa, ss);
                }
            }
        }
    }

}
