extern crate shmem;

use std::thread;
use std::time::Duration;

#[derive(Copy, Clone)]
#[derive(Debug)]
struct Foo {
    bar: u32,
    baz: u32,
}

fn main() {
    let mut foo = shmem::create::<Foo, _>("shmem-rust-test").unwrap();

    foo.bar = 12;
    foo.baz = 34;

    let mut bar = shmem::array::create::<u8, _>("shmem-rust-array", 10).unwrap();
    for (i, item) in bar.iter_mut().enumerate() {
        *item = i as u8;
    }

    thread::sleep(Duration::from_secs(40));

}