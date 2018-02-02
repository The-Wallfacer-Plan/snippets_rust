#![feature(use_nested_groups)]
#![feature(box_syntax)]
extern crate rsgenetic;

use rsgenetic::pheno::{Phenotype, Fitness};
use rsgenetic::sim::{seq::Simulator, Simulation, Builder, select::*};

#[derive(Eq, PartialEq, Ord, PartialOrd)]
struct MyFitness {
    value: i32,
}

impl Fitness for MyFitness {
    fn zero() -> Self {
        MyFitness { value: 0 }
    }

    fn abs_diff(&self, other: &Self) -> Self {
        MyFitness {
            value: (self.value - other.value).abs()
        }
    }
}

const TARGRT: i32 = 100;

#[derive(Copy, Clone, Debug)]
struct MyPheno {
    x: i32,
    y: i32,
}

impl Phenotype<MyFitness> for MyPheno {
    fn fitness(&self) -> MyFitness {
        MyFitness { value: (self.x + self.y) - TARGRT }
    }

    fn crossover(&self, other: &Self) -> Self {
        MyPheno {
            x: self.x,
            y: other.y,
        }
    }

    fn mutate(&self) -> Self {
        MyPheno {
            x: self.x + 1,
            y: self.y - 1,
        }
    }
}

fn ga(iter: u64) {
    println!("\nmax_iter={}", iter);
    let delta = MyFitness { value: 10 };
    let mut population = (0..30).map(|i| MyPheno { x: i, y: 100 - i }).collect();
    let builder = Simulator::builder(&mut population)
        .set_selector(box TournamentSelector::new(12, 6))
        .set_early_stop(delta, 50);
    let mut s = builder.set_max_iters(iter).build();
    s.run();
    let result = s.get();
    match result {
        Ok(sim) => {
            println!("{:?}, fitness={}", sim, sim.fitness().value);
        }
        Err(e) => {
            eprintln!("error: {:?}", e);
        }
    }
}

fn main() {
    for i in 1..20 {
        ga(i * 200);
    }
}