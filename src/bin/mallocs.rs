extern crate libc;

use libc::*;

// Assume we have a C function that returns a malloc'ed ptr.
unsafe extern "C" fn create_str() -> *mut c_char {
    let ptr = malloc(12) as *mut c_char;
    strcpy(ptr, b"Hello world\0".as_ptr() as *const c_char);
    ptr
}

unsafe extern "C" fn del_str(ptr: *mut c_char) {
    free(ptr as *mut c_void);
}

use std::slice;
fn main() {
    // we wrap the null-terminated ptr into an Mptr.
    let ptr = unsafe { create_str() };
    {
        let s = unsafe { slice::from_raw_parts(ptr, 12) };
        println!("{:?}", s);
    }
    unsafe {
        del_str(ptr);
    }
    println!("{:?}", ptr);
}
