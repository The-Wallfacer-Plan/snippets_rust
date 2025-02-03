#![allow(unused_variables, dead_code, unused_imports)]

use std::cell::Cell;
use std::cell::RefCell;

pub fn cells() {
    let x = Cell::new(1);
    let y = &x;
    let z = &x;
    x.set(2);
    y.set(3);
    z.set(4);
    println!("{}", x.get());
}

fn simples() {
    let mut x = 1;
    let &mut y = &mut x;
    let &mut z = &mut x;
    x = 2;
    // y = 3;
    // z = 4;
    println!("{}", x);
}

fn main() {
    cells();
}

