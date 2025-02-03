#![allow(unused_variables, dead_code, unused_imports)]
#![feature(test)]
extern crate test;

extern crate fxhash;
extern crate twox_hash;

use fxhash::FxHasher;
use std::collections::hash_map::DefaultHasher;
use std::hash::Hasher;
use twox_hash::XxHash32;
use twox_hash::XxHash64;

fn hasher_bench<H>(b: &mut test::Bencher, mut hasher: H, len: usize)
where
    H: Hasher,
{
    let bytes: Vec<_> = (0..100).cycle().take(len).collect();
    b.bytes = bytes.len() as u64;
    b.iter(|| {
        hasher.write(&bytes);
        hasher.finish()
    });
}

fn main() {}

#[inline]
fn xxhash_bench(b: &mut test::Bencher, len: usize) {
    hasher_bench(b, XxHash64::with_seed(0), len)
}

#[inline]
fn xxhash32_bench(b: &mut test::Bencher, len: usize) {
    hasher_bench(b, XxHash32::with_seed(0), len)
}

#[inline]
fn fxhash_bench(b: &mut test::Bencher, len: usize) {
    hasher_bench(b, FxHasher::default(), len)
}

const LEN: usize = 1 << 16;

#[bench]
fn bench_xxhash(b: &mut test::Bencher) {
    xxhash_bench(b, LEN);
}

#[bench]
fn bench_xxhash32(b: &mut test::Bencher) {
    xxhash32_bench(b, LEN);
}

#[bench]
fn bench_fxhash(b: &mut test::Bencher) {
    fxhash_bench(b, LEN);
}
