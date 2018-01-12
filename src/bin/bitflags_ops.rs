#[macro_use]
extern crate bitflags;

bitflags! {
    struct Flags: u32 {
        const FLAG_0       = 0b00000000;
        const FLAG_A       = 0b00000001;
        const FLAG_B       = 0b00000010;
        const FLAG_C       = 0b00000100;
        const FLAG_ABC     = Flags::FLAG_A.bits
                           | Flags::FLAG_B.bits
                           | Flags::FLAG_C.bits;
    }
}

fn main() {
    let e1 = Flags::FLAG_A | Flags::FLAG_C;
    let e2 = Flags::FLAG_B | Flags::FLAG_C;
    assert_eq!((e1 | e2), Flags::FLAG_ABC); // union
    assert_eq!((e1 & e2), Flags::FLAG_C); // intersection
    assert_eq!((e1 - e2), Flags::FLAG_A); // set difference
    assert_eq!(!e2, Flags::FLAG_A); // set complement
    if e1 | e2 != Flags::FLAG_0 {
        println!("{:?}", e1 & e2);
    }
}
