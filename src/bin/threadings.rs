#![allow(unused_variables, dead_code, unused_imports)]
use std::thread::sleep;
use std::thread::spawn;
use std::time::Duration;

extern crate num_cpus;
extern crate threadpool;

fn passing_type(n_jobs: usize) {
    use std::sync::mpsc;
    use std::thread;
    use threadpool::ThreadPool;

    let n_workers = num_cpus::get();
    let pool = ThreadPool::new(n_workers);

    let (tx, rx): (mpsc::Sender<f64>, mpsc::Receiver<f64>) = mpsc::channel();

    #[inline]
    fn term(k: usize) -> f64 {
        let c: i8 = if k & 1 == 0 { 1 } else { -1 };
        ((4 * c) as f64) / ((2 * k + 1) as f64)
    }

    for i in 0..n_jobs {
        let tx = tx.clone();
        pool.execute(move || {
            tx.send(term(i)).unwrap();
        });
    }

    let res: f64 = rx.iter().take(n_jobs).sum();
    println!("res={}", res);
}

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

    spawn(move || {
        for i in 0..10 {
            let thread_tx = tx.clone();
            thread_tx.send((i + 2) * 4).unwrap();
        }
    });

    for _ in 0..10 {
        println!("recv: {}", rx.recv().unwrap());
    }
}

fn main() {
    // channels();
    // let n = 4967296;
    // passing_type(n);
}
