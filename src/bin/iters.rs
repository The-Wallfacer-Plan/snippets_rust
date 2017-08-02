fn main() {
    let arr = [0i32, 3, -8, 9, -10];
    let m = arr.iter()
        .enumerate()
        .max_by_key(|&(_, &x)| x.abs())
        .unwrap();
    println!("{:?}", m);
}
