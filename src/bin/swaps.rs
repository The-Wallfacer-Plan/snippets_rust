fn main() {
    let vu16 = 0x1234_u16;
    println!("{:x}", vu16.swap_bytes());
    let vu32 = 0x12345678_u32;
    println!("{:x}", vu32.swap_bytes());
}
