use std::time::Instant;
use rand::{rngs::StdRng, Rng, SeedableRng};
use rayon::prelude::*;

fn main() {
    let n: usize = 2000;

    let mut a: Vec<Vec<f64>> = vec![vec![0.0_f64; n]; n];
    let mut b: Vec<Vec<f64>> = vec![vec![0.0_f64; n]; n];
    let mut c: Vec<Vec<f64>> = vec![vec![0.0_f64; n]; n];

    // Equivalent to `srand(42)` in C
    let mut rng = StdRng::seed_from_u64(42);

    for i in 0..n {
        for j in 0..n {
            a[i][j] = rng.random::<u32>() as f64;
            b[i][j] = rng.random::<u32>() as f64;
        }
    }

    let start_time = Instant::now();

    c.par_iter_mut()
        .enumerate()
        .for_each(|(i, row_c)| {
            for j in 0..row_c.len() {
                let mut total = 0.0;
                for k in 0..row_c.len() {
                    total += a[i][k] * b[k][j];
                }
                row_c[j] = total;
            }
        });


    let duration = start_time.elapsed().as_secs_f64();
    println!("{:.6} seconds", duration);

    // Suppress compiler optimization by using the result.
    let checksum: f64 = c[0].iter().take(8).sum();
    eprintln!("checksum(first row, 8) = {:.6e}", checksum);
}
