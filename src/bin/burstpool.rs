extern crate threadpool;
extern crate burst_pool;

use burst_pool::BurstPool;
use threadpool::ThreadPool;
use std::sync::mpsc::channel;
use std::sync::mpsc;
use std::thread;
use std::time::Duration;
use std::collections::VecDeque;

struct MutMgr {}

impl MutMgr {
    fn run(sender: mpsc::Sender<i32>, receiver: mpsc::Receiver<i32>) {
        let mut mut_pool = BurstPool::new();
        let mut send_pool = BurstPool::new();

        let (tx, rx): (mpsc::Sender<i32>, mpsc::Receiver<i32>) = channel();


        // receive
        thread::spawn(move ||{
            let mut count = 0;
            loop {
                if let Ok(item) = receiver.try_recv() {
                    let tx = tx.clone();
                    mut_pool.spawn(move |item| {
                        println!("MutMgr received: {}", item);
                        thread::sleep(Duration::new(1, 0));
                        let mutated = item + 1;
                        tx.send(mutated).unwrap();
                    });
                    mut_pool.send(item);
                    count += 1;
                }
            }
            if count >= 50 {
                panic!("MutMgr ending!");
            } 
        });

        

        // send
        loop {
            if let Ok(item) = rx.try_recv() {
                let sender = sender.clone();
                send_pool.spawn(move |item| {
                    println!("MutMgr sending: {}", item);
                    sender.send(item).unwrap();
                });
                send_pool.send(item);
            }
        }
    }
}

struct RunMgr {}

impl RunMgr {
    fn run(sender: mpsc::Sender<i32>, receiver: mpsc::Receiver<i32>) {
        let mut mut_pool = BurstPool::new();
        let mut send_pool = BurstPool::new();

        let (tx, rx): (mpsc::Sender<i32>, mpsc::Receiver<i32>) = channel();


        // receive
        thread::spawn(move ||{
            let mut count = 0;
            loop {
                if let Ok(item) = receiver.try_recv() {
                    let tx = tx.clone();
                    mut_pool.spawn(move |item| {
                        println!("RunMgr received: {}", item);
                        thread::sleep(Duration::new(3, 0));
                        let mutated = item + 1;
                        tx.send(mutated).unwrap();
                    });
                    mut_pool.send(item);
                    count += 1;
                }
            }
            if count >= 50 {
                panic!("RunMgr ending!");
            } 
        });

        

        // send
        loop {
            if let Ok(item) = rx.try_recv() {
                let sender = sender.clone();
                send_pool.spawn(move |item| {
                    println!("RunMgr sending: {}", item);
                    sender.send(item).unwrap();
                });
                send_pool.send(item);
            }
        }
    }
}


struct TraceMgr {}

impl TraceMgr {
    fn run(sender: mpsc::Sender<i32>, receiver: mpsc::Receiver<i32>) {
        let mut mut_pool = BurstPool::new();
        let mut send_pool = BurstPool::new();

        let (tx, rx): (mpsc::Sender<i32>, mpsc::Receiver<i32>) = channel();


        // receive
        thread::spawn(move ||{
            let mut count = 0;
            loop {
                if let Ok(item) = receiver.try_recv() {
                    let tx = tx.clone();
                    mut_pool.spawn(move |item| {
                        println!("TraceMgr received: {}", item);
                        thread::sleep(Duration::new(1, 0));
                        let mutated = item + 1;
                        tx.send(mutated).unwrap();
                    });
                    mut_pool.send(item);
                    count += 1;
                }
            }
            if count >= 50 {
                panic!("TraceMgr ending!");
            } 
        });

        

        // send
        loop {
            if let Ok(item) = rx.try_recv() {
                let sender = sender.clone();
                send_pool.spawn(move |item| {
                    println!("TraceMgr sending: {}", item);
                    sender.send(item).unwrap();
                });
                send_pool.send(item);
            }
        }
    }
}

fn main() {
    let mut g_q = VecDeque::with_capacity(10);
    let mut i = 0;
    while i < 1000 {
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
    let tra = thread::spawn(move || { TraceMgr::run(to_mutator_sender, tracer_receiver); });


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
