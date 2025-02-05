use std::thread;
use std::thread::JoinHandle;

fn main() {
    const N: u8 = 16;

    let (mut tx, rx) = spmc::channel();

    let mut handles: Vec<JoinHandle<()>> = Vec::new();
    for n in 0..N {
        let rx = rx.clone();
        handles.push(thread::spawn(move || {
            let msg = rx.recv().unwrap();
            println!("receiver {} received: {}", n, msg);
        }));
    }

    for i in 0..N {
        let msg = i * 2;
        println!("sender {} sends {}", i, msg);
        tx.send(msg).unwrap();
    }

    for handle in handles {
        handle.join().unwrap();
    }
}
