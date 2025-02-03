extern "C" fn before_main() {
    println!("Hello, world!");
}

#[unsafe(link_section = ".init_array")]
#[used]
static INIT_ARRAY: [extern "C" fn(); 1] = [before_main];

fn main() {}
