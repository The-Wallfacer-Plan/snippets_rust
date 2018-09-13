#![allow(unused_variables, dead_code, unused_imports)]
#![feature(test)]
extern crate test;

use std::ptr;
use std::u16;

type T = u16;

const SIZE: usize = u16::MAX as usize;

fn main() {
    //
}

pub fn create_with_collect() -> Vec<T> {
    (0..SIZE as T).collect::<Vec<T>>()
}

pub fn create_manually() -> Vec<T> {
    let mut arr = vec![0; SIZE];
    for (n, b) in arr.iter_mut().enumerate() {
        *b = n as T;
    }
    arr
}

pub fn create_unsafe() -> Vec<T> {
    let mut arr = Vec::with_capacity(SIZE);
    unsafe {
        arr.set_len(SIZE);
    }

    for (n, mut b) in arr.iter_mut().enumerate() {
        unsafe {
            ptr::write(b, n as T);
        }
    }
    arr
}

#[cfg(test)]
mod tests {
    use super::*;

    #[bench]
    fn bench_collect(b: &mut test::Bencher) {
        b.iter(|| create_with_collect());
    }

    #[bench]
    fn bench_manually(b: &mut test::Bencher) {
        b.iter(|| create_manually());
    }

    #[bench]
    fn bench_unsafe(b: &mut test::Bencher) {
        b.iter(|| create_unsafe());
    }
}
