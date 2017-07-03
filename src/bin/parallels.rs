#![allow(dead_code, unused_variables)]
extern crate rayon;

use rayon::prelude::*;

fn main() {
    #[inline]
    fn term(k: usize) -> f64 {
        let c: i8 = if k & 1 == 0 { 1 } else { -1 };
        ((4 * c) as f64) / ((2 * k + 1) as f64)
    }

    use rayon::iter::ParallelIterator;
    fn pi(n: usize) -> f64 {
        (0..n).into_par_iter().map(term).sum()
    }

    {
        fn spi(n: usize) -> f64 {
            (0..n).into_iter().map(term).sum()
        }
        let v = spi(std::u32::MAX as usize);
        println!("res={}", v);
    }

    // let v = pi(std::u32::MAX as usize);
    // println!("res={}", v);

}

fn qsort() {
    let mut input = (0..1000).collect::<Vec<_>>();

    // Calculate the sum of squares
    let sq_sum: i32 = input.par_iter().map(|&i| i * i).sum();

    // Increment each element in parallel
    input.par_iter_mut().for_each(|p| *p += 1);

    // Parallel quicksort
    let mut input = (0..1000).rev().collect::<Vec<_>>();
    quick_sort(&mut input);
}

fn quick_sort<T: PartialOrd + Send>(v: &mut [T]) {
    if v.len() <= 1 {
        return;
    }

    let mid = partition(v);
    let (lo, hi) = v.split_at_mut(mid);
    rayon::join(|| quick_sort(lo), || quick_sort(hi));
}

fn partition<T: PartialOrd + Send>(v: &mut [T]) -> usize {
    let pivot = v.len() - 1;
    let mut i = 0;
    for j in 0..pivot {
        if v[j] <= v[pivot] {
            v.swap(i, j);
            i += 1;
        }
    }
    v.swap(i, pivot);
    i
}
