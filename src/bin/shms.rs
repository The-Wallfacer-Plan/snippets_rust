// #![feature(rand, plugin)]
#![allow(non_snake_case, unused_variables, dead_code, unused_imports)]
extern crate errno;
extern crate libc;
extern crate rand;

// use rand::{Rng, thread_rng};

use std::env;
use std::u32;
use std::u16;

const ENV_VAR: &str = "TEST_SHM";

use std::thread;
use std::ptr;

use std::ffi::CString;
use std::ffi::CStr;

use std::time;

const SIZE: usize = 20;

unsafe fn str_to_cchar(s: &str) -> CString {
    CString::new(s).unwrap()
}

fn get_shm_id(key: libc::c_int) -> i32 {
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

struct Child {
    info: u32,
}

impl Child {
    fn new(info: u32) -> Self {
        Child { info: info }
    }

    fn run(&self) {
        println!("in child run");
        let shm_id_str = env::var(ENV_VAR).expect("cannot find ENV_VAR");
        let shm_id = shm_id_str.as_str().parse().unwrap();
        println!("unsafe... {}", shm_id);
        unsafe {
            println!("unsafe...");
            let pa: *mut libc::c_char = attach_shm(shm_id);
            let ss: *const libc::c_char = str_to_cchar("bad").as_ptr();
            println!("--ss...");
        }
    }
}

fn main() {
    // let ref mut threadRng = thread_rng();
    let key = 0 as libc::key_t;
    // let key = threadRng.gen_range(0, u32::MAX/ 2) as i32;
    let shm_id = get_shm_id(key);

    println!("key={}, shm_id={}", key, shm_id);

    // env::set_var(ENV_VAR, shm_id.to_string());

    // let child = thread::spawn(move || {
    //     let c = Child::new(42);
    //     println!("enter child, info={}", c.info);
    //     c.run();
    //     println!("leave child");
    // });

    // thread::sleep(time::Duration::from_millis(1000));

    // child.join().expect("success join");
    // println!("back!");

    unsafe {
        let pa: *mut libc::c_char = attach_shm(shm_id);
        let cs: CString = str_to_cchar("bad");
        let ss: *const libc::c_char = cs.as_ptr();
        let cstr = CStr::from_ptr(ss).to_string_lossy().into_owned();
        println!("=={}|", cstr);
        libc::strcpy(pa, ss);
        println!("lol");
        let slice = CStr::from_ptr(pa);
        println!("{:?}", slice);
    }

    remove_shm(shm_id);
}
