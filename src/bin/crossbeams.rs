extern crate crossbeam;

struct OtherStruct {
    ms: MyStruct,
}

struct MyStruct {
    u: u8,
    s: String,
}

fn mutex_ver() {
    //
}

fn cb_ver() {
    //
}

fn main() {
    let ms: MyStruct = unsafe { ::std::mem::uninitialized() };
    let mut os = OtherStruct { ms: ms };
    os.ms = MyStruct {
        u: 12,
        s: "good".to_string(),
    };
}
