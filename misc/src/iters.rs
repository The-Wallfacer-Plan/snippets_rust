#![allow(unused_variables, dead_code, unused_imports)]
#![allow(clippy::uninit_vec)]
#![feature(test)]
extern crate test;

use std::ptr;

type IntTy = u16;

const SIZE: usize = u16::MAX as usize;

fn main() {
    //
}

pub fn create_with_collect() -> Vec<IntTy> {
    (0..SIZE as IntTy).collect::<Vec<IntTy>>()
}

pub fn create_manually() -> Vec<IntTy> {
    let mut arr = vec![0; SIZE];
    for (n, b) in arr.iter_mut().enumerate() {
        *b = n as IntTy;
    }
    arr
}

pub fn create_unsafe() -> Vec<IntTy> {
    let mut arr = Vec::with_capacity(SIZE);
    unsafe {
        arr.set_len(SIZE);
    }

    for (n, b) in arr.iter_mut().enumerate() {
        unsafe {
            ptr::write(b, n as IntTy);
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
