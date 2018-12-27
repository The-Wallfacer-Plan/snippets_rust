#![feature(used)]

extern "C" fn before_main() {
    println!("Hello, world!");
}

#[link_section = ".init_array"]
#[used]
static INIT_ARRAY: [extern "C" fn(); 1] = [before_main];

fn main() {}
