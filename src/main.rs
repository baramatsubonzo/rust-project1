use std::time::Instant;
use rand::Rng;

const N: usize = 1500;

fn main() {
    let mut rng = rand::thread_rng();

    let mut a: Vec<Vec<f64>> = vec![vec![0.0; N]; N];
    let mut b: Vec<Vec<f64>> = vec![vec![0.0; N]; N];
    let mut c: Vec<Vec<f64>> = vec![vec![0.0; N]; N];

    for i in 0..N {
        for j in 0..N {
            a[i][j] = rng.random::<f64>();
            b[i][j] = rng.random::<f64>();
        }
    }

    let start_time = Instant::now();

    for i in 0..N {
        for j in 0..N {
            let mut total = 0.0;
            for k in 0..N {
                total += a[i][k] * b[k][j];
            }
            c[i][j] = total;
        }
    }

    let duration = start_time.elapsed();
    println!("{:.2?} seconds", duration);
}