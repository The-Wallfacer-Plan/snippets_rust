fn main() {
    let x = 0b0u16;
    println!("{}, {}", x.trailing_zeros(), x & 0b1111 == 0);
}
