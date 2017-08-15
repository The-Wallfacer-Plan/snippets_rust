use std::boxed::Box;

trait MyTrait {}

struct MyStruct;

impl MyTrait for MyStruct {}

fn foo(t: &MyTrait) {
    //
}

fn main() {
    let t: Box<MyTrait> = Box::new(MyStruct);
    foo(&*t);
}
