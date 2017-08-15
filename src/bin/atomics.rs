use std::sync::atomic::AtomicBool;
use std::sync::Arc;
use std::sync::atomic::Ordering;
use std::{thread, time};

fn main() {
    let mut ab = Arc::new(AtomicBool::new(false));
    let abc = ab.clone();
    let handler = thread::spawn(move || loop {
        let duration = time::Duration::from_millis(1000);
        thread::sleep(duration);
        println!("refcount={}", Arc::strong_count(&abc));
        if abc.load(Ordering::Relaxed) {
            break;
        }
    });
    // ab.store(true, Ordering::SeqCst);
    ab = Arc::new(AtomicBool::new(true));
    handler.join().unwrap();
}
