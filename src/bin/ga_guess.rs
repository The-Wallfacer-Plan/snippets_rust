#![feature(use_nested_groups)]
#![feature(box_syntax)]
#![feature(swap_nonoverlapping)]
extern crate rsgenetic;
extern crate rand;

use rand::Rng;
use rsgenetic::pheno::{Phenotype, Fitness};
use rsgenetic::sim::{seq::Simulator, Simulation, Builder, select::*};

const LEN: usize = 20;
const TARGET: &[u8; LEN] = b"Hello, World!\\^_=_^/";

#[derive(Clone, Debug)]
struct Bytes {
    v: [u8; LEN],
}

impl Bytes {
    pub fn new(v: [u8; LEN]) -> Bytes {
        Bytes { v }
    }
}

impl Phenotype<i32> for Bytes {
    fn fitness(&self) -> i32 {
        let v = self.v.iter().zip(TARGET.iter()).fold(0i32, |acc, (vi, ti)| {
            let abs = if vi > ti { vi - ti } else { ti - vi };
            acc + (abs as i32)
        });
        return -v;
    }

    fn crossover(&self, other: &Self) -> Self {
//        let pivot = LEN / 2;
//        use std::ptr;
//        let mut x = self.clone();
//        let mut y = other.clone();
//        unsafe {
//            ptr::swap_nonoverlapping(x.v.as_mut_ptr(), y.v.as_mut_ptr(), pivot);
//        }
//        return x;
        self.clone()
    }

    fn mutate(&self) -> Self {
        let mut rnd = rand::weak_rng();
        let idx = rnd.gen_range(0, LEN);
        let v = rnd.gen_range(0x32, 0x7F);
        let mut mutated = self.clone();
        mutated.v[idx] = v;
        return mutated;
    }
}

fn main() {
    let iter = 500000u64;
    let mut population = vec![
        Bytes::new([55u8; LEN]),
        Bytes::new([37; LEN]),
        Bytes::new([59; LEN]),
        Bytes::new([73; LEN]),
        Bytes::new([99; LEN]),
        Bytes::new([70; LEN]),
    ];
    let delta = 3i32;
    let selector_res = TournamentSelector::new_checked(2, 2);
    match selector_res {
        Ok(selector) => {
            let builder = Simulator::builder(&mut population)
                .set_selector(box selector)
                .set_early_stop(delta, iter / 2);
            let mut s = builder.set_max_iters(iter).build();
            s.run();
            let result = s.get();
            match result {
                Ok(sim) => {
                    println!("{:?}, fitness={}", sim, sim.fitness());
                }
                Err(e) => {
                    eprintln!("error: {:?}", e);
                }
            }
            println!("{:?}", TARGET);
        }
        Err(e) => {
            eprintln!("error: {:?}", e);
        }
    }
}