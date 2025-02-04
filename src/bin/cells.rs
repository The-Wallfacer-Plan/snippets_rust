use std::cell::Cell;
use std::cell::RefCell;

pub fn cells() {
    println!("==> {}", stdext::function_name!());
    let x = Cell::new(1);
    let y = &x;
    let z = &x;
    x.set(2);
    y.set(3);
    z.set(4);
    println!("x:{:?}, y:{:?}, z:{:?}", x.get(), y.get(), z.get());

    let p = Person::default();
    println!("{:?}", p);
    p.celebrate_birthday();
    println!("{:?}", p);
}

pub fn refcells() {
    let c = RefCell::new("hello".to_owned());
    {
        let mut content = c.borrow_mut();
        *content = "world".to_owned();
    }
    println!("{:?}", c.borrow());
}

#[derive(Default, Debug)]
struct Person {
    age: Cell<u8>,
    name: String,
}

impl Person {
    fn celebrate_birthday(&self) {
        self.age.set(self.age.get() + 1);
    }
}

fn main() {
    cells();
    refcells();
}
