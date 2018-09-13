#![feature(test)]

extern crate crossbeam;
extern crate hwloc;
extern crate libc;
extern crate rand;
extern crate test;

use rand::distributions::{IndependentSample, Range};
use std::thread;
use test::Bencher;
use hwloc::{CpuSet, ObjectType, Topology, CPUBIND_THREAD};
use crossbeam::scope;

/// Example on how to check for specific topology support of a feature.
fn check_support() {
    let topo = Topology::new();

    // Check if Process Binding for CPUs is supported
    println!(
        "CPU Binding (current process) supported: {}",
        topo.support().cpu().set_current_process()
    );
    println!(
        "CPU Binding (any process) supported: {}",
        topo.support().cpu().set_process()
    );

    // Check if Thread Binding for CPUs is supported
    println!(
        "CPU Binding (current thread) supported: {}",
        topo.support().cpu().set_current_thread()
    );
    println!(
        "CPU Binding (any thread) supported: {}",
        topo.support().cpu().set_thread()
    );

    // Check if Memory Binding is supported
    println!(
        "Memory Binding supported: {}",
        topo.support().memory().set_current_process()
    );

    // Debug Print all the Support Flags
    println!("All Flags:\n{:?}", topo.support());
}

fn main() {
    check_support();
}

/// get cpu idx for cpu binding
fn cpuset_for_core(topology: &Topology, idx: usize) -> CpuSet {
    let cores = (*topology).objects_with_type(&ObjectType::Core).unwrap();
    match cores.get(idx) {
        Some(val) => val.cpuset().unwrap(),
        None => panic!("No Core found with id {}", idx),
    }
}

#[bench]
fn no_binding(b: &mut Bencher) {
    scope(|scope| {
        b.iter(move || {
            let between = Range::new(-1f64, 1.);
            let mut rng = rand::StdRng::new().unwrap();
            let total = 1_000_000u32;
            let mut in_circle = 0u32;
            for _ in 0u32..total {
                let a = between.ind_sample(&mut rng);
                let b = between.ind_sample(&mut rng);
                if a * a + b * b <= 1. {
                    in_circle += 1;
                }
            }
        });
    });
}

#[bench]
fn binding(b: &mut Bencher) {
    scope(|scope| {
        // cpu binding logic
        let mut topo = Topology::new();
        let tid = unsafe { libc::pthread_self() };
        // load the cpuset for the given core index.
        let mut bind_to = cpuset_for_core(&topo, 0 as usize);
        // Get only one logical processor (in case the core is SMT/hyper-threaded).
        bind_to.singlify();
        topo.set_cpubind_for_thread(tid, bind_to, CPUBIND_THREAD)
            .unwrap();
        let binded_idx = topo.get_cpubind_for_thread(tid, CPUBIND_THREAD).unwrap();

        b.iter(move || {
            let between = Range::new(-1f64, 1.);
            let mut rng = rand::StdRng::new().unwrap();
            let total = 1_000_000u32;
            let mut in_circle = 0u32;
            for _ in 0u32..total {
                let a = between.ind_sample(&mut rng);
                let b = between.ind_sample(&mut rng);
                if a * a + b * b <= 1. {
                    in_circle += 1;
                }
            }
        });
    });
}
