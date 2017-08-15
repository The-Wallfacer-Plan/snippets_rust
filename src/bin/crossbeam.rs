#![feature(test)]
#![feature(cfg_target_feature)]

extern crate crossbeam;
extern crate simd;
extern crate test;

use simd::u8x16;
use crossbeam::{Scope, scope};
use crossbeam::sync::SegQueue;
use std::u8;
use std::ops::{BitAnd, BitOr};
use test::Bencher;

fn main() {
    // let queue = SegQueue::new();
    const MAP_SIZE: usize = 8;
    let mut virgin_bits = Box::new([u8x16::splat(u8::MAX); MAP_SIZE]);
    // let trace_bits = Box::new([u8x16::splat(0u8); MAP_SIZE]);

    scope(|scope| {
        for mut i in &mut (*virgin_bits) {
            scope.spawn(move || {
                println!("prev element: {:?}", i);
                *i = (*i).bitand(u8x16::splat(35u8));
                // *i &= u8x16::splat(0u8); cannot use &= ????
                println!("after element: {:?}", i);
            });
        }

    });

    // single thread test
    for mut i in &mut (*virgin_bits).iter_mut() {
        println!("\tprev element: {:?}", i);
        *i = (*i).bitor(u8x16::splat(u8::MAX));
        println!("\tafter element: {:?}", i);
    }
}

#[bench]
fn naive(b: &mut Bencher) {

    b.iter(move || {
        const MAP_SIZE: usize = 50;
        let mut virgin_bits = Box::new([[u8::MAX; 16]; MAP_SIZE]);
        // let queue = SegQueue::new();
        // let trace_bits = Box::new([u8x16::splat(0u8); MAP_SIZE]);

        scope(|scope| {
            for mut i in &mut (*virgin_bits).iter_mut() {
                scope.spawn(move || {
                    // println!("prev element: {:?}", i);
                    // *i = i.bitand(u8x16::splat(0u8));
                    for j in 0..16 {
                        i[j] &= 35u8;
                    }
                    // *i &= u8x16::splat(0u8); cannot use &= ????
                    // println!("after element: {:?}", i);
                });
            }

        });
    });
}

#[bench]
fn simd_u8x16(b: &mut Bencher) {
    b.iter(move || {
        const MAP_SIZE: usize = 50;
        let mut virgin_bits = Box::new([u8x16::splat(u8::MAX); MAP_SIZE]);
        // let queue = SegQueue::new();
        // let trace_bits = Box::new([u8x16::splat(0u8); MAP_SIZE]);

        scope(|scope| {
            for mut i in &mut (*virgin_bits).iter_mut() {
                scope.spawn(move || {
                    // println!("prev element: {:?}", i);
                    *i = (*i).bitand(u8x16::splat(35u8));
                    // *i &= u8x16::splat(0u8); cannot use &= ????
                    // println!("after element: {:?}", i);
                });
            }

        });
    });
}

#[bench]
fn naive_single_thread(b: &mut Bencher) {

    b.iter(move || {
        const MAP_SIZE: usize = 50;
        let mut virgin_bits = Box::new([[u8::MAX; 16]; MAP_SIZE]);

        for mut i in &mut (*virgin_bits).iter_mut() {
            for j in 0..16 {
                i[j] &= 35u8;
            }
        }

    });
}

// >= 50 ----> non-zero, <50 ----> zero ns
#[bench]
fn simd_u8x16_single_thread(b: &mut Bencher) {
    b.iter(move || {
        const MAP_SIZE: usize = 50;
        let mut virgin_bits = Box::new([u8x16::splat(u8::MAX); MAP_SIZE]);

        for mut i in &mut (*virgin_bits).iter_mut() {
            *i = (*i).bitand(u8x16::splat(35u8));
        }

    });
}
