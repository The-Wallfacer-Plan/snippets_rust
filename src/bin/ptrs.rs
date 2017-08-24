fn main() {
    let mut b = Box::new(22);
    innocent_looking_fn(&b);
    *b += 1;
    println!("{}", *b);
}

fn innocent_looking_fn(b: &Box<usize>) {
    // This wicked little bit of code will take a borrowed
    // `Box` and free it.
    unsafe {
        let p: *const usize = &**b;
        let q: Box<usize> = Box::from_raw(p as *mut usize);
    }
}
