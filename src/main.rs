use std::time::Instant;
use rand::{rngs::StdRng, Rng, SeedableRng};
use rayon::prelude::*;

#[inline]
fn idx(i: usize, j: usize, n: usize) -> usize { i * n + j } // row-major

fn main() {
    let n: usize = 2000;

    // Flattened matrix in contiguous memory
    let mut a = vec![0.0_f64; n * n];
    let mut bt = vec![0.0_f64; n * n];
    let mut c = vec![0.0_f64; n * n];

    // Equivalent to `srand(42)` in C
    let mut rng = StdRng::seed_from_u64(42);


    // Initialize metrices and transpose matrix b for better cache performance
    let t0 = Instant::now();
    for i in 0..n {
        // row i of A
        for j in 0..n {
            a[idx(i, j, n)] = rng.random::<u32>() as f64;
        }
        // Generate row i of B and store it into BT transposed (BT[j, i] = B[i, j])
        for j in 0..n {
            let bij = rng.random::<u32>() as f64;
            bt[idx(j, i, n)] = bij; // store B[i, j] into BT[j, i]
        }
    }
    let t_transpose = t0.elapsed().as_secs_f64();

    // Start timing the matrix multiplication
    let start_time = Instant::now();

    c.par_chunks_mut(n)
        .enumerate()
        .for_each(|(i, row_c)| {
            let ai = &a[i * n .. i * n + n];
            for j in 0..row_c.len() {
                let btj = &bt[j * n .. j * n + n];
                let mut sum = 0.0;
                for k in 0.. n {
                    sum += ai[k] * btj[k];
                }
                row_c[j] = sum;
            }

        });

    let duration = start_time.elapsed().as_secs_f64();
    println!("multiply only: {:.6} seconds", duration);
    println!("transpose(B): {:.6} seconds", t_transpose);

     // Suppress compiler optimization by using the result.
    let checksum: f64 = c[0..8].iter().sum();

    eprintln!("checksum(first row, 8) = {:.6e}", checksum);
}
