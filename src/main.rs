// thanks to κeen
extern crate ndarray;
extern crate rand;

use ndarray::prelude::*;
use rand::{random, Rng, XorShiftRng};
use std::time::Instant;

#[inline]
fn ising2d_sum_of_adjacent_spins(s: &Array2<i64>, m: usize, n: usize, i: usize, j: usize) -> i64 {
    let i_bottom = if i + 1 < m { i + 1 } else { 0 };
    let i_top = if i >= 1 { i - 1 } else { m - 1 };
    let j_right = if j + 1 < n { j + 1 } else { 0 };
    let j_left = if j >= 1 { j - 1 } else { n - 1 };
    return s[(i_bottom, j)] + s[(i_top, j)] + s[(i, j_right)] + s[(i, j_left)];
}

fn ising2d_sweep(mut s: Array2<i64>, beta: f64, niter: i64) {
    let m = s.shape()[0];
    let n = s.shape()[1];
    let prob: Vec<f64> = (-4..5).map(|s| (-2.0 * beta * s as f64).exp()).collect();
    let mut rng = XorShiftRng::new_unseeded();
    let iteration = niter / (m * n) as i64;
    for _ in 0..iteration {
        for i in 0..m {
            for j in 0..n {
                let s1 = s[(i, j)];
                let k = s1 * ising2d_sum_of_adjacent_spins(&s, m, n, i, j);
                let val = rng.gen_range(0., 1.);
                if val < prob[(k + 4) as usize] {
                    s[(i, j)] = -(s1 as i64);
                }
            }
        }
    }
}

fn main() {
    let n = 100;
    let mut s = Array::<i64, _>::zeros((n, n));
    for i in 0..n {
        for j in 0..n {
            let val: f64 = random();
            s[(i, j)] = if val < 0.5 { -1 } else { 1 };
        }
    }

    let niter = 1000_000_000;
    let beta = (1.0 + 2.0_f64.sqrt()).ln() / 2.0;
    let start = Instant::now();
    ising2d_sweep(s, beta, niter);
    let end = start.elapsed();
    println!(
        "{}.{:03}秒経過しました。",
        end.as_secs(),
        end.subsec_nanos() / 1_000_000
    );
}
