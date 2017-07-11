#![allow(unused_variables, dead_code, unused_imports)]
use std::thread::spawn;
use std::thread::sleep;
use std::time::Duration;

extern crate threadpool;
extern crate num_cpus;

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
        pool.execute(move || { tx.send(term(i)).unwrap(); });
    }

    let res: f64 = rx.iter().take(n_jobs).sum();
    println!("res={}", res);

}

fn use_thread_pool() {
    use threadpool::ThreadPool;
    use std::sync::mpsc::channel;
    use std::sync::mpsc;
    use std::thread;
    use std::time::Duration;

    let n_workers = num_cpus::get() / 2;
    let n_jobs = num_cpus::get() - 2;
    let pool = ThreadPool::new(n_workers);

    let (tx, rx): (mpsc::Sender<i32>, mpsc::Receiver<i32>) = channel();
    // pool.execute(move || {
    //     let mut i = 0;
    //     loop {
    //         i += 2;
    //         thread::sleep(Duration::new(1, 0));
    //         tx.send(i).unwrap();
    //     }
    // });
    // // pool.execute(move || loop {
    //     let received = rx.recv().unwrap();
    //     println!("got {}", received);
    // });
    ///
    let (tx1, rx1): (mpsc::Sender<i32>, mpsc::Receiver<i32>) = channel();
    thread::spawn(move || {
        // let mut i = 0;
        let (tx3, rx3): (mpsc::Sender<i32>, mpsc::Receiver<i32>) = channel();
        let pool1 = ThreadPool::new(10);
        // loop {
        // i += 3;
        thread::spawn(move || loop {
            let received = rx.recv().unwrap();
            println!("got {}", received);
            pool1.execute(move || {
                println!("runner run!!!!");
                tx3.send(received).unwrap();
            });
        });
        thread::spawn(move || {
            let received = rx3.recv().unwrap();

            let mut i = 0;
            loop {
                tx1.send(received).unwrap();
                thread::sleep(Duration::new(1, 0));
            }
        });
        // let received = rx.recv().unwrap();
        // println!("got {}", received);
        // thread::sleep(Duration::new(0, 500000000));
        // }
    });
    // thread::sleep(Duration::new(1, 0));
    thread::spawn(move || {
        let pool2 = ThreadPool::new(2);
        pool2.execute(move || loop {
            let received = rx1.recv().unwrap();
            println!("have {}", received);
        });
        pool2.execute(move || {
            let mut i = 0;
            loop {
                i += 2;
                tx.send(i).unwrap();
                thread::sleep(Duration::new(0, 500000000));
            }
        });
    });
    ///
    loop {
        println!("main");
        thread::sleep(Duration::new(1, 0));
    }
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

    spawn(move || for i in 0..10 {
        let thread_tx = tx.clone();
        thread_tx.send((i + 2) * 4).unwrap();
    });

    for _ in 0..10 {
        println!("recv: {}", rx.recv().unwrap());
    }
}

fn main() {
    // channels();
    use_thread_pool();
    // let n = 4967296;
    // passing_type(n);
}
