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

struct MutMgr {}

impl MutMgr {
    fn run(sender: mpsc::Sender<i32>, receiver: mpsc::Receiver<i32>) {
        let mut count = 0;
        let mut_pool = ThreadPool::new(10);
        let send_pool = ThreadPool::new(10);
        // let mut mut_pool = BurstPool::new();
        // let mut send_pool = BurstPool::new();

        let (tx, rx): (mpsc::Sender<i32>, mpsc::Receiver<i32>) = channel();

        // receive
        thread::spawn(move || loop {
            // match receiver.try_iter().next() {
            //     Some(item) => {
            //         let mut count = 0;
            //         for entry in receiver.iter() {
            //             let tx = tx.clone();
            //             mut_pool.execute(move || {
            //                 thread::sleep(Duration::new(0, 100000000));
            //                 println!("MutMgr received: {}", entry);
            //                 let mutated = entry + 1;
            //                 tx.send(mutated).unwrap();
            //             });
            //             count += 1;
            //         }
            //         println!("MutMgr received {} entries", count);
            //     },
            //     None => {}
            // }
            println!(
                "mut pool: {}/{}",
                mut_pool.active_count(),
                mut_pool.max_count()
            );
            if let Ok(item) = receiver.try_recv() {
                let tx = tx.clone();
                mut_pool.execute(move || {
                    println!("MutMgr received: {}", item);
                    // thread::sleep(Duration::new(0, 300_000_000));
                    thread::sleep(Duration::new(1, 0));
                    let mutated = item + 1;
                    tx.send(mutated).unwrap();
                })
                // mut_pool.spawn(move |item| {
                //     println!("MutMgr received: {}", item);
                //     thread::sleep(Duration::new(0, 100000000));
                //     let mutated = item + 1;
                //     tx.send(mutated).unwrap();
                // });
                // mut_pool.send(item);
            }

        });

        // send
        loop {
            if let Ok(item) = rx.try_recv() {
                let sender = sender.clone();
                send_pool.execute(move || {
                    println!("MutMgr sending: {}", item);
                    sender.send(item).unwrap();
                });
                count += 1;
                // send_pool.spawn(move |item| {
                //     println!("MutMgr sending: {}", item);
                //     sender.send(item).unwrap();
                // });
                // send_pool.send(item);
            }
            if count >= 100 {
                panic!("MutMgr ends");
            }
        }
    }
}

struct RunMgr {}

impl RunMgr {
    fn run(sender: mpsc::Sender<i32>, receiver: mpsc::Receiver<i32>) {
        let mut_pool = ThreadPool::new(30);
        let send_pool = ThreadPool::new(30);
        let (tx, rx): (mpsc::Sender<i32>, mpsc::Receiver<i32>) = channel();
        let mut count = 0;
        // receive
        thread::spawn(move || {

            loop {
                println!(
                    "run pool: {}/{}",
                    mut_pool.active_count(),
                    mut_pool.max_count()
                );
                match receiver.try_recv() {
                    Ok(item) => {
                        let tx = tx.clone();
                        mut_pool.execute(move || {
                            // let mut rng = rand::thread_rng();
                            println!("RunMgr received: {}", item);
                            // thread::sleep(Duration::new(0, rng.gen_range::<u32>(300000000u32, 600000000u32)));
                            thread::sleep(Duration::new(3, 0));
                            let mutated = item + 1;
                            tx.send(mutated).unwrap();
                        });
                    }
                    _ => {}
                }
            }
        });

        // send
        loop {
            match rx.try_recv() {
                Ok(item) => {
                    let sender = sender.clone();
                    send_pool.execute(move || {
                        println!("RunMgr sending: {}", item);
                        sender.send(item).unwrap();
                    });
                    count += 1;
                }
                _ => {}
            }
            if count >= 100 {
                panic!("RunMgr ends");
            }
        }
    }
}


struct TraceMgr {}

impl TraceMgr {
    fn run(sender: mpsc::Sender<i32>, receiver: mpsc::Receiver<i32>) {
        let mut_pool = ThreadPool::new(10);
        let send_pool = ThreadPool::new(10);
        let (tx, rx): (mpsc::Sender<i32>, mpsc::Receiver<i32>) = channel();
        let mut count = 0;
        // receive
        thread::spawn(move || loop {
            println!(
                "trace pool: {}/{}",
                mut_pool.active_count(),
                mut_pool.max_count()
            );
            match receiver.try_recv() {
                Ok(item) => {
                    let tx = tx.clone();
                    mut_pool.execute(move || {
                        println!("TraceMgr received: {}", item);
                        thread::sleep(Duration::new(1, 0));
                        let mutated = item + 1;
                        tx.send(mutated).unwrap();
                    });
                }
                _ => {}
            }
        });

        // send
        loop {
            match rx.try_recv() {
                Ok(item) => {
                    let sender = sender.clone();
                    send_pool.execute(move || {
                        println!("TraceMgr sending: {}", item);
                        sender.send(item).unwrap();
                    });
                    count += 1;
                }
                _ => {}
            }
            if count >= 100 {
                panic!("TraceMgr ends!");
            }
        }
    }
}

fn main() {
    let mut g_q = VecDeque::with_capacity(10);
    let mut i = 0;
    while i < 300 {
        g_q.push_back(i);
        i += 10;
    }

    let (to_mutator_sender, mutator_receiver): (mpsc::Sender<i32>, mpsc::Receiver<i32>) = channel();
    let (to_runner_sender, runner_receiver): (mpsc::Sender<i32>, mpsc::Receiver<i32>) = channel();
    let (to_tracer_sender, tracer_receiver): (mpsc::Sender<i32>, mpsc::Receiver<i32>) = channel();
    let (to_main_sender, main_receiver): (mpsc::Sender<i32>, mpsc::Receiver<i32>) = channel();

    // initial queue provided by the user
    while !g_q.is_empty() {
        let item = g_q.pop_front().unwrap();
        to_mutator_sender.send(item).unwrap();
    }

    let mutr = thread::spawn(move || { MutMgr::run(to_runner_sender, mutator_receiver); });
    let run = thread::spawn(move || { RunMgr::run(to_tracer_sender, runner_receiver); });
    let tra = thread::spawn(move || {
        TraceMgr::run(to_mutator_sender, tracer_receiver);
    });


    mutr.join();
    run.join();
    tra.join();

    // loop {
    //     // let received = main_receiver.recv().unwrap();
    //     // println!("received from main: {}", received);
    //     // g_q.push_back(received);
    //     // let item = g_q.pop_front().unwrap();
    //     // println!("qsize={}, sending from main: {}", g_q.len(), item);
    //     // to_mutator_sender.send(item).unwrap();
    // }
}
