#![allow(unused_variables, dead_code)]
use std::thread::spawn;
use std::thread::sleep;
use std::time::Duration;

fn shared() {
    let s = "world";
    let child = spawn(move || {
        println!("Hello {}", s);
    });
    let duration = Duration::from_millis(100);
    println!("hello {} from main", s);
    sleep(duration);

    child.join().unwrap();

}

fn channels() {
    use std::sync::mpsc;
    let (tx, rx) = mpsc::channel();

    spawn(move || for i in 0..10 {
        let thread_tx = tx.clone();
        thread_tx.send((i + 2) * 4).unwrap();
    });

    for _ in 0..10 {
        println!("recv: {}", rx.recv().unwrap());
    }
}

fn main() {
    channels();
}
