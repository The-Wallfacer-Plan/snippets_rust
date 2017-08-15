use std::thread;
use std::time::Duration;
use std::sync::mpsc::{self, TryRecvError};
use std::io::{self, BufRead};

fn main() {
    println!("Press enter to wake up the child thread");
    let (tx, rx) = mpsc::channel();
    thread::spawn(move || loop {
        println!("Suspending...");
        match rx.recv() {
            Ok(_) => {
                println!("Working...");
                thread::sleep(Duration::from_millis(500));
            }
            Err(_) => {
                println!("Terminating.");
                break;
            }
        }
    });

    let mut line = String::new();
    let stdin = io::stdin();
    for _ in 0..4 {
        let _ = stdin.lock().read_line(&mut line);
        let _ = tx.send(());
    }
}

fn terminate() {
    println!("Press enter to terminate the child thread");
    let (tx, rx) = mpsc::channel();
    thread::spawn(move || loop {
        println!("Working...");
        thread::sleep(Duration::from_millis(500));
        match rx.try_recv() {
            Ok(_) |
            Err(TryRecvError::Disconnected) => {
                println!("Terminating.");
                break;
            }
            Err(TryRecvError::Empty) => {}
        }
    });

    let mut line = String::new();
    let stdin = io::stdin();
    let _ = stdin.lock().read_line(&mut line);

    let _ = tx.send(());
}
