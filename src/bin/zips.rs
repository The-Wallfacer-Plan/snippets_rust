use std::cmp;

fn naive_dot_product(x: &[f64], y: &[f64]) -> f64 {
    x.iter().zip(y.iter()).fold(
        0.0,
        |sum, (&ex, &ey)| sum + (ex * ey),
    )
}

// The method you describe.
fn index_dot_product(x: &[f64], y: &[f64]) -> f64 {
    let n = cmp::min(x.len(), y.len());
    let (x, y) = (&x[..n], &y[..n]);
    let mut sum = 0.0;
    for i in 0..n {
        sum += x[i] * y[i];
    }
    sum
}

// Shift slices in place and add 8 elements at a time.
fn unrolled_dot_product(x: &[f64], y: &[f64]) -> f64 {
    let n = cmp::min(x.len(), y.len());
    let (mut x, mut y) = (&x[..n], &y[..n]);

    let mut sum = 0.0;
    while x.len() >= 8 {
        sum += x[0] * y[0] + x[1] * y[1] + x[2] * y[2] + x[3] * y[3] + x[4] * y[4] +
            x[5] * y[5] + x[6] * y[6] + x[7] * y[7];
        x = &x[8..];
        y = &y[8..];
    }

    // Take care of any left over elements (if len is not divisible by 8).
    x.iter().zip(y.iter()).fold(
        sum,
        |sum, (&ex, &ey)| sum + (ex * ey),
    )
}

fn main() {}
