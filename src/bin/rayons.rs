#![feature(test)]

#[cfg(test)]
mod tests {
    use rayon::iter::IntoParallelIterator;
    use rayon::iter::ParallelIterator;

    #[inline]
    fn pi_calc_iter(k: usize) -> f64 {
        let c: i8 = if k & 1 == 0 { 1 } else { -1 };
        ((4 * c) as f64) / ((2 * k + 1) as f64)
    }

    extern crate test;
    // make this value bigger to view bigger performance differences
    const NUM: usize = (std::u16::MAX as usize) * 1024;
    #[bench]
    fn bench_pi_seq(b: &mut test::Bencher) {
        b.iter(|| {
            let n = test::black_box(NUM);
            (0..n).into_iter().map(pi_calc_iter).sum::<f64>()
        })
    }

    #[bench]
    fn bench_pi_par(b: &mut test::Bencher) {
        b.iter(|| {
            let n = test::black_box(NUM);
            (0..n).into_par_iter().map(pi_calc_iter).sum::<f64>()
        })
    }
}

fn par_calculate() {
    use rayon::iter::IntoParallelIterator;
    use rayon::iter::IntoParallelRefIterator;
    use rayon::iter::IntoParallelRefMutIterator;
    use rayon::iter::ParallelIterator;
    let mut input = (0..32u32).collect::<Vec<_>>();

    // Calculate the sum of squares
    let sq_sum: u32 = input.par_iter().map(|&i| i * i).sum();
    println!("{}", sq_sum);

    // Increment each element in parallel(input is mutated)
    input.par_iter_mut().for_each(|p| *p += 1);
    println!("{:?}", input);

    // Parallel quicksort(orignal input is mutated)
    input.reverse();
    println!("{:?}", input);
    quick_sort(&mut input);
    println!("{:?}", input);
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

fn main() {
    par_calculate();
}
