//#![allow(unused_features)]
#![allow(unused_extern_crates, dead_code, unused_mut)]
#![allow(unused_assignments, unused_variables, unused_imports, unreachable_code)]
//#![feature(alloc_system)]
//#![feature(repr_simd, platform_intrinsics, cfg_target_feature)]
//#![feature(vec_remove_item)]
#![feature(box_syntax)]
#![feature(core_intrinsics, plugin, test, splice, const_fn)]
#![feature(placement_in_syntax)]
#![cfg_attr(feature = "clippy", plugin(clippy))]
#![cfg_attr(feature = "clippy", allow(needless_return, needless_range_loop, needless_lifetimes, bool_comparison, too_many_arguments, match_same_arms))]
#![cfg_attr(feature = "clippy", allow(let_and_return, collapsible_if, block_in_if_condition_stmt, let_unit_value, needless_pass_by_value, if_same_then_else))]
#![cfg_attr(feature = "clippy", allow(verbose_bit_mask))]
//extern crate alloc_system;
#[macro_use]
extern crate pretty_assertions;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate serde_json;
#[macro_use]
extern crate log;
extern crate test;
extern crate log4rs;
extern crate toml;
extern crate nix;
extern crate libc;
extern crate bytecount;
extern crate rand;
extern crate chrono;
extern crate memmem;
extern crate redis;
#[macro_use]
extern crate lazy_static;
extern crate hwloc;
#[macro_use]
extern crate bitflags;
#[macro_use]
extern crate maplit;
extern crate fxhash;
extern crate twox_hash;

const V: u8 = 2;
const SIZE: usize = 1 << 16;

fn main() {
    let mut v: Vec<u8> = vec![1, 2, 3];
    v.swap_remove(2);
    println!("{:?}", v);
    let b = box [V; SIZE];
    println!("{}", b[32]);
}
