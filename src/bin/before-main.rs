use stdext::function_name;

extern "C" fn before_main() {
    println!("inside {}", function_name!());
}

#[unsafe(link_section = ".init_array")]
#[used]
static INIT_ARRAY: [extern "C" fn(); 1] = [before_main];

fn main() {
    println!("inside {}", function_name!());
}
