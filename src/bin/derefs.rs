#![allow(clippy::needless_borrow)]

struct X {
    val: i32,
}
impl std::ops::Deref for X {
    type Target = i32;
    fn deref(&self) -> &i32 {
        &self.val
    }
}

trait M {
    fn m(self);
}
impl M for i32 {
    fn m(self) {
        println!("i32::m()");
    }
}
impl M for X {
    fn m(self) {
        println!("X::m()");
    }
}
impl M for &X {
    fn m(self) {
        println!("&X::m()");
    }
}
impl M for &&X {
    fn m(self) {
        println!("&&X::m()");
    }
}
impl M for &&&X {
    fn m(self) {
        println!("&&&X::m()");
    }
}

trait RefM {
    fn refm(&self);
}
impl RefM for i32 {
    fn refm(&self) {
        println!("i32::refm()");
    }
}
impl RefM for X {
    fn refm(&self) {
        println!("X::refm()");
    }
}
impl RefM for &X {
    fn refm(&self) {
        println!("&X::refm()");
    }
}
impl RefM for &&X {
    fn refm(&self) {
        println!("&&X::refm()");
    }
}
impl RefM for &&&X {
    fn refm(&self) {
        println!("&&&X::refm()");
    }
}

struct Y {
    val: i32,
}
impl std::ops::Deref for Y {
    type Target = i32;
    fn deref(&self) -> &i32 {
        &self.val
    }
}

struct Z {
    val: Y,
}
impl std::ops::Deref for Z {
    type Target = Y;
    fn deref(&self) -> &Y {
        &self.val
    }
}

#[derive(Clone)]
struct A;
impl std::marker::Copy for A {}
impl M for A {
    fn m(self) {
        println!("A::m()");
    }
}
impl M for &&&A {
    fn m(self) {
        println!("&&&A::m()");
    }
}
impl RefM for A {
    fn refm(&self) {
        println!("A::refm()");
    }
}
impl RefM for &&&A {
    fn refm(&self) {
        println!("&&&A::refm()");
    }
}

fn main() {
    // I'll use @ to denote left side of the dot operator
    (*X { val: 42 }).m(); // i32::refm() , self == @
    X { val: 42 }.m(); // X::m()      , self == @
    (&X { val: 42 }).m(); // &X::m()     , self == @
    (&&X { val: 42 }).m(); // &&X::m()    , self == @
    (&&&X { val: 42 }).m(); // &&&X:m()    , self == @
    (&&&&X { val: 42 }).m(); // &&&X::m()   , self == *@
    (&&&&&X { val: 42 }).m(); // &&&X::m()   , self == **@

    (*X { val: 42 }).refm(); // i32::refm() , self == @
    X { val: 42 }.refm(); // X::refm()   , self == @
    (&X { val: 42 }).refm(); // X::refm()   , self == *@
    (&&X { val: 42 }).refm(); // &X::refm()  , self == *@
    (&&&X { val: 42 }).refm(); // &&X::refm() , self == *@
    (&&&&X { val: 42 }).refm(); // &&&X::refm(), self == *@
    (&&&&&X { val: 42 }).refm(); // &&&X::refm(), self == **@

    Y { val: 42 }.refm(); // i32::refm() , self == *@
    Z { val: Y { val: 42 } }.refm(); // i32::refm() , self == **@

    A.m(); // A::m()      , self == @
           // without the Copy trait, (&A).m() would be a compilation error:
           // cannot move out of borrowed content
    (&A).m(); // A::m()      , self == *@
    (&&A).m(); // &&&A::m()   , self == &@
    (&&&A).m(); // &&&A::m()   , self == @
    A.refm(); // A::refm()   , self == @
    (&A).refm(); // A::refm()   , self == *@
    (&&A).refm(); // A::refm()   , self == **@
    (&&&A).refm(); // &&&A::refm(), self == @
}
