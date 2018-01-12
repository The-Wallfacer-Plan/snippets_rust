fn main() {
    let mut arr = [1u8, 2, 3, 4];
    {
        let pos: &u8 = unsafe { arr.get_unchecked(2) };
        let v = pos + 1u8;
        println!("v={}, pos={}, *pos={}", v, pos, *pos);
    }
    {
        let pos: &mut u8 = unsafe { arr.get_unchecked_mut(2) };
        // let v = pos + 1u8;
    }
}
