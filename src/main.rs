use std::time::Instant;
use rand::Rng;

fn main() {
    let n: usize = 1500;

    let mut rng = rand::thread_rng();

    let mut a: Vec<Vec<f64>> = vec![vec![0.0; n]; n];
    let mut b: Vec<Vec<f64>> = vec![vec![0.0; n]; n];
    let mut c: Vec<Vec<f64>> = vec![vec![0.0; n]; n];

    for i in 0..n {
        for j in 0..n {
            a[i][j] = rng.random::<f64>();
            b[i][j] = rng.random::<f64>();
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

    let duration = start_time.elapsed();
    println!("{:.6?} seconds", duration);
}