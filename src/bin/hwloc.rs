#![feature(test)]
#![feature(cfg_target_feature)]

extern crate hwloc;
extern crate libc;
extern crate rand;
extern crate test;
extern crate crossbeam;

use rand::distributions::{IndependentSample, Range};
use std::thread;
use test::Bencher;
use hwloc::{CPUBIND_THREAD, CpuSet, ObjectType, Topology};
use crossbeam::{Scope, scope};

// fn main() {

// }

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
            let total = 1_000_000_0u32;
            let mut in_circle = 0u32;
            for _ in 0u32..total {
                let a = between.ind_sample(&mut rng);
                let b = between.ind_sample(&mut rng);
                if a*a + b*b <= 1. {
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
        topo
            .set_cpubind_for_thread(tid, bind_to, CPUBIND_THREAD)
            .unwrap();
        let binded_idx = topo
            .get_cpubind_for_thread(tid, CPUBIND_THREAD)
            .unwrap();
    
        b.iter(move || {
            let between = Range::new(-1f64, 1.);
            let mut rng = rand::StdRng::new().unwrap();
            let total = 1_000_000_0u32;
            let mut in_circle = 0u32;
            for _ in 0u32..total {
                let a = between.ind_sample(&mut rng);
                let b = between.ind_sample(&mut rng);
                if a*a + b*b <= 1. {
                    in_circle += 1;
                }
            }
        });
    });
}