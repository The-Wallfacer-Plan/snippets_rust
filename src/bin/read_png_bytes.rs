use std::fmt::Debug;
use std::fs::File;
use std::io::{Error, Lines, BufReader, BufRead, Read};
use std::path::Path;

fn main() {
    // 'IH'
    let ts_key_l: u16 = (b'I' as u16) * 256 + (b'H' as u16);
    // 'HI'
    let ts_key_b: u16 = (b'H' as u16) * 256 + (b'I' as u16);
    let mut bytes = read_file_bytes("examples/c_not_kitty.png");

    for i in 0..(bytes.len()-1) {
        unsafe {
            let ts = * (as_ptr16(bytes.as_mut_ptr(), i));
            if ts == ts_key_l {
                println!("found IH");
            }
            if ts == ts_key_b {
                println!("found HI");
            }
        }
    }

    for byte in bytes {
        print!("{:x} ", byte);
    }
    println!();
}

#[inline]
pub fn as_ptr16(ptr: *mut u8, i: usize) -> *const u16 {
    unsafe { ptr.offset(i as isize) as *const u16 }
}

#[inline]
pub fn read_file_bytes<P: AsRef<Path> + Debug>(fpath: P) -> Vec<u8> {
    read_file_bytes_result(&fpath).unwrap_or_else(|e| panic!("cannot read file {:?}, {:?}", fpath, e))
}

#[inline]
pub fn read_file_bytes_result<P: AsRef<Path> + Debug>(fpath: P) -> Result<Vec<u8>, Error> {
    let mut file = File::open(&fpath)?;
    let mut bytes = Vec::new();
    file.read_to_end(&mut bytes)?;
    Ok(bytes)
}
