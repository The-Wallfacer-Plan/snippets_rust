extern crate libc;

use std::fs::{self, File, OpenOptions};
use std::io::{stdout, Read, Seek, SeekFrom, Write};
use std::thread;
use std::time::Duration;
use libc::*;
use std::os::unix::io::AsRawFd;

fn main() {
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
        f.flush();
    }
}
