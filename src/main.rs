use std::time::Instant;
use rand::{rngs::StdRng, Rng, SeedableRng};

#[inline]
fn idx(i: usize, j: usize, n: usize) -> usize { i * n + j } // row-major

fn main() {
    let n: usize = 1500;

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
            bt[idx(j, i, n)] = bij; // B[i, j]
        }
    }
    let t_transpose = t0.elapsed().as_secs_f64();

    // Start timing the matrix multiplication
    let start_time = Instant::now();

    for i in 0..n {
        let ai = &a[idx(i, 0, n) .. idx(i, 0, n) + n]; // row i of A
        for j in 0..n {
            let btj = &bt[idx(j, 0, n) .. idx(j, 0, n) + n]; // row j of BT (col j of B)
            let mut sum = 0.0;
            for k in 0..n {
                sum += ai[k] * btj[k];  // A[i, k] * B[k, j]
            }
            c[idx(i, j, n)] = sum; // C[i, j]
        }
    }

    let duration = start_time.elapsed().as_secs_f64();
    println!("multiply only: {:.6?} seconds", duration);
    println!("transpose(B): {:.6?} seconds", t_transpose);

    // Use part of the result matrix (checksum of the first row)
    // to prevent the compiler from optimizing away the entire
    // matrix multiplication as dead code. This ensures that
    // the benchmark measures the actual computation cost.
    let checksum: f64 = c[0..8].iter().sum();
    eprintln!("checksum(first row, 8) = {:.6e}", checksum);
}
