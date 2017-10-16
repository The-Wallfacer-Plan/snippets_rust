use std::collections::VecDeque;

fn v1() {
    let mut v = vec![1, 2, 3];
    let mut h = v[0];
    // let h = v.first().unwrap();
    v.push(4);
    v.push(5);
    h = 0;
    println!("{}, {:?}", h, v);
}

fn v2() {
    let mut v = VecDeque::new();
    for i in 1..4 {
        v.push_back(i);
    }
    let mut h = v[0];
    // let h = v.front().unwrap();
    v.push_back(4);
    v.push_back(5);
    h = 0;
    println!("{}, {:?}", h, v);
}

fn test(v: &mut Vec<usize>) -> usize {
    let e = v.pop().unwrap();
    e + v.len()
}

struct Stat {
    i: u8,
    s: String,
}

fn main() {
    let mut v = vec![1, 2, 3];
    let res = test(&mut v);
    let l = v.len();
    println!("{}, {}", res, l);
}
