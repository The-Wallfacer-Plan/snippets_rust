#![allow(dead_code, unused_imports)]
extern crate libc;

use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering;
use std::sync::Arc;
use std::{thread, time};

use libc::*;
use std::fs::{self, File, OpenOptions};
use std::io::{Read, Seek, SeekFrom, Write};
use std::os::unix::io::AsRawFd;
use std::time::Duration;

fn t_ptrs() {
    fn innocent_looking_fn(b: &Box<usize>) {
        // This wicked little bit of code will take a borrowed
        // `Box` and free it.
        unsafe {
            let p: *const usize = &**b;
            let _q: Box<usize> = Box::from_raw(p as *mut usize);
        }
    }

    let mut b = Box::new(22);
    innocent_looking_fn(&b);
    *b += 1;
    println!("{}", *b);
}

fn t_fwrite() {
    let mut f = OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .read(true)
        .open("/tmp/test_file")
        .unwrap();
    {
        let v1 = b"good";
        f.write_all(v1).unwrap();
        //        f.flush();
    }

    let t = Duration::from_secs(2);
    thread::sleep(t);

    {
        let v2 = b"bad";
        {
            let fd = f.as_raw_fd();
            let len = v2.len() as off_t;
            unsafe {
                lseek(fd, 0, SEEK_SET);
                ftruncate(fd, len);
            }
        }

        f.write_all(v2).unwrap();
        f.flush().unwrap();
    }
}

fn t_atomics() {
    let mut ab = Arc::new(AtomicBool::new(false));
    let abc = ab.clone();
    let handler = thread::spawn(move || loop {
        let duration = time::Duration::from_millis(1000);
        thread::sleep(duration);
        println!("refcount={}", Arc::strong_count(&abc));
        if abc.load(Ordering::Relaxed) {
            break;
        }
    });
    // ab.store(true, Ordering::SeqCst);
    ab = Arc::new(AtomicBool::new(true));
    handler.join().unwrap();
}

fn t_swaps() {
    let vu16 = 0x1234_u16;
    println!("{:x}", vu16.swap_bytes());
    let vu32 = 0x12345678_u32;
    println!("{:x}", vu32.swap_bytes());
}

fn t_to_string() {
    #[inline(never)]
    fn foo_aaa(s: String) {
        println!("{}", s);
    }

    #[inline(never)]
    fn foo_bbb(s: String) {
        println!("{}", s.to_string());
    }

    let sssaaa = String::from("aaaaaa");
    let sssbbb = String::from("bbbbbb");
    foo_aaa(sssaaa);
    foo_bbb(sssbbb);
}

fn t_envs() {
    if let Ok(s) = std::env::var("CC") {
        println!("CC={}", s);
    } else {
        println!("CC not set");
    }
}

fn main() {
    //    t_atomics();
    t_envs();
}

