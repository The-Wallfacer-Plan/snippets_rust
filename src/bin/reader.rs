#![allow(unused_variables, dead_code, unused_imports)]
extern crate shmem;

#[derive(Copy,Clone)]
pub struct Foo {
    bar:u32,
    baz:u32
}

fn main() {
    let foo = shmem::open::<Foo, _>("shmem-rust-test").unwrap();
    println!("bar={}, baz={}", foo.bar, foo.baz);

    let bar = shmem::array::open::<u8, _>("shmem-rust-array").unwrap();
    println!("{:?}", &*bar);
}