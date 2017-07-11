extern crate threadpool;
extern crate burst_pool;
extern crate rand;

use burst_pool::BurstPool;
use threadpool::ThreadPool;
use std::sync::mpsc::channel;
use std::sync::mpsc;
use std::thread;
use std::time::Duration;
use std::collections::VecDeque;
use rand::{Rng, thread_rng};

struct Fuzz {}

impl Fuzz {
    fn run(sender: mpsc::Sender<i32>, receiver: mpsc::Receiver<i32>) {
        let mut count = 0;
        let mut_pool = ThreadPool::new(70);
        let send_pool = ThreadPool::new(70);

        let (tx, rx): (mpsc::Sender<i32>, mpsc::Receiver<i32>) = channel();

        // receive
        thread::spawn(move || loop {
            if let Ok(item) = receiver.try_recv() {
                let tx = tx.clone();
                mut_pool.execute(move || {
                    println!("MutMgr received: {}", item);
                    thread::sleep(Duration::new(7, 0));
                    tx.send(item).unwrap();
                });
            }
        });

        // send
        loop {
            if let Ok(item) = rx.try_recv() {
                let sender = sender.clone();
                send_pool.execute(move || {
                    println!("Fuzz sending: {}", item);
                    sender.send(item).unwrap();
                });
                count += 1;
            }
            if count >= 100 {
                sender.send(-1).unwrap();
                panic!("Fuzz ends");
            }
        }
    }
}

fn main() {
    let mut g_q : VecDeque<i32> = VecDeque::new();
    let mut i = 0;
    while i < 700 {
        g_q.push_back(i);
        i += 10;
    }

    let (to_fuzz_sender, fuzz_receiver): (mpsc::Sender<i32>, mpsc::Receiver<i32>) = channel();
    let (to_main_sender, main_receiver): (mpsc::Sender<i32>, mpsc::Receiver<i32>) = channel();

    // initial queue provided by the user
    while !g_q.is_empty() {
        let item = g_q.pop_front().unwrap();
        to_fuzz_sender.send(item).unwrap();
    }

    let fuzz = thread::spawn(move || { Fuzz::run(to_main_sender, fuzz_receiver); });

    loop {
        while let Some(item) = main_receiver.try_iter().next() {
            if item < 0 {
                panic!("Main thread ends");
            } 
            g_q.push_back(item);
            println!("received from main: {}, now queue size is {}", item, g_q.len());
        }
        if let Some(item) = g_q.pop_front() {
            println!("qsize={}, sending from main", g_q.len());
            to_fuzz_sender.send(item).unwrap();
        }
    }

}
