#![feature(test)]
#![feature(cfg_target_feature)]

extern crate crossbeam;
extern crate simd;
extern crate test;

use simd::u8x16;
use crossbeam::{Scope, scope};
use crossbeam::sync::SegQueue;
use std::u8;
use std::ops::BitAnd;
use test::Bencher;

fn main() {
    // let queue = SegQueue::new();
    const MAP_SIZE: usize = 4;
    let mut virgin_bits = Box::new([u8x16::splat(u8::MAX); MAP_SIZE]);
    // let trace_bits = Box::new([u8x16::splat(0u8); MAP_SIZE]);

    scope(|scope| {
        for mut i in &mut (*virgin_bits) {
            scope.spawn(move || {
                println!("prev element: {:?}", i);
                *i = i.bitand(u8x16::splat(0u8));
                // *i &= u8x16::splat(0u8); cannot use &= ????
                println!("after element: {:?}", i);
            });
        }

    });
}

#[bench]
fn simd_u8x16(b: &mut Bencher) {
    const MAP_SIZE: usize = 4;
    let mut virgin_bits = Box::new([u8x16::splat(u8::MAX); MAP_SIZE]);
    b.iter(move || {
        // let queue = SegQueue::new();
        // let trace_bits = Box::new([u8x16::splat(0u8); MAP_SIZE]);

        scope(|scope| {
            for mut i in &mut (*virgin_bits) {
                scope.spawn(move || {
                    // println!("prev element: {:?}", i);
                    *i = i.bitand(u8x16::splat(0u8));
                    // *i &= u8x16::splat(0u8); cannot use &= ????
                    // println!("after element: {:?}", i);
                });
            }

        });
    });
}

#[bench]
fn naive(b: &mut Bencher) {

    const MAP_SIZE: usize = 4;
    let mut virgin_bits = Box::new([[u8::MAX; 16]; MAP_SIZE]);
    b.iter(move || {
        // let queue = SegQueue::new();
        // let trace_bits = Box::new([u8x16::splat(0u8); MAP_SIZE]);

        scope(|scope| {
            for mut i in &mut (*virgin_bits) {
                scope.spawn(move || {
                    // println!("prev element: {:?}", i);
                    // *i = i.bitand(u8x16::splat(0u8));
                    for j in 0..16 {
                        i[j] &= 0u8;
                    }
                    // *i &= u8x16::splat(0u8); cannot use &= ????
                    // println!("after element: {:?}", i);
                });
            }

        });
    });
}

