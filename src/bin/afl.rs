extern crate threadpool;

use threadpool::ThreadPool;
use std::sync::mpsc::channel;
use std::sync::mpsc;
use std::thread;
use std::time::Duration;
use std::collections::VecDeque;
use std::sync::Arc;
use std::sync::Mutex;
use std::cell::RefCell;

struct MutMgr {}

impl MutMgr {
    fn run(r_entry: mpsc::Receiver<i32>, s_mutated: mpsc::Sender<i32>) {
        let mut out_q: VecDeque<i32> = VecDeque::new();
        let mut out_q_cm = Arc::new(RefCell::new(out_q));
        // let mut out_q = Arc::new(out_q);
        // let out_q_c = Arc::make_mut(&mut out_q);
        let mut out_q_c = out_q_cm.clone();
        // let mut out_q_cm = out_q_c.clone();
        // let out_q_c = Arc::make_mut(&mut out_q_m);
        // let out_q_c = Arc::make_mut(&mut out_q);
        let mut_pool = ThreadPool::new(10);
        thread::spawn(move || {
            let out_q_cc = out_q_c.clone();
            loop {
                let out_q_ccc = out_q_cc.clone();
                let received_entry = r_entry.recv().unwrap();
                mut_pool.execute(move || {
                    println!("MutMgr received entry: {}", received_entry);
                    thread::sleep(Duration::new(0, 500000000));
                    out_q_ccc.get_mut().push_back(received_entry);
                });
            }
        });
        loop {
            match out_q_cm.get_mut().pop_front() {
                Some(e) => {
                    s_mutated.send(e).unwrap();
                }
                None => {}
            }
        }
    }
}



fn main() {
    let mut g_q = VecDeque::with_capacity(10);
    for i in 0..10 {
        g_q.push_back(i);
    }
    let (tx, rx): (mpsc::Sender<i32>, mpsc::Receiver<i32>) = channel();
    // MutMgr::run()


}
