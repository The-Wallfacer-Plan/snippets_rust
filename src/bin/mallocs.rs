extern crate libc;

use libc::{c_char, malloc, strcpy};

// Assume we have a C function that returns a malloc'ed string.
unsafe extern "C" fn create_str() -> *mut c_char {
    let ptr = malloc(12) as *mut c_char;
    strcpy(ptr, b"Hello world\0".as_ptr() as *const c_char);
    ptr
}

fn main() {
    // we wrap the null-terminated string into an MString.
    let string = unsafe { create_str() };

    println!("{:?}", string);


    // the string will be dropped by `free` after the code is done.
}
