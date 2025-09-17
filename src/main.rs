use std::time::Instant;
use rand::{rngs::StdRng, Rng, SeedableRng};

fn main() {
    let n: usize = 1500;

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

    for i in 0..n {
        for j in 0..n {
            let mut total = 0.0;
            for k in 0..n {
                total += a[i][k] * b[k][j];
            }
            c[i][j] = total;
        }
    }

    let duration = start_time.elapsed().as_secs_f64();
    println!("{:.6?} seconds", duration);
}