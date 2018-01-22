//thanks to κeen
extern crate ndarray;
extern crate rand;

use ndarray::prelude::*;
use rand::{random, Open01, Rng, XorShiftRng};
use std::time::Instant;

fn ising2d_sum_of_adjacent_spins(s: &Array2<i64>, m: usize, n: usize, i: usize, j: usize) -> i64 {
    let i_bottom: usize;
    let i_top: usize;
    let j_right: usize;
    let j_left: usize;
    if i + 1 < m {
        i_bottom = i + 1
    } else {
        i_bottom = 0
    }
    if i >= 1 {
        i_top = i - 1
    } else {
        i_top = m - 1
    }
    if j + 1 < n {
        j_right = j + 1
    } else {
        j_right = 0
    }
    if j >= 1 {
        j_left = j - 1
    } else {
        j_left = n - 1
    }
    return s[(i_bottom, j)] + s[(i_top, j)] + s[(i, j_right)] + s[(i, j_left)];
}

fn ising2d_sweep(mut s: Array2<i64>, beta: f64, niter: i64) {
    let m = s.shape()[0];
    let n = s.shape()[1];
    let prob = [
        f64::exp(-2.0 * beta * (-4.0)),
        f64::exp(-2.0 * beta * (-3.0)),
        f64::exp(-2.0 * beta * (-2.0)),
        f64::exp(-2.0 * beta * (-1.0)),
        f64::exp(-2.0 * beta * (0.0)),
        f64::exp(-2.0 * beta * (1.0)),
        f64::exp(-2.0 * beta * (2.0)),
        f64::exp(-2.0 * beta * (3.0)),
        f64::exp(-2.0 * beta * (4.0)),
    ];
    let mut rng = XorShiftRng::new_unseeded();
    let iteration = niter / (m * n) as i64;
    for _ in 0..iteration {
        for i in 0..m {
            for j in 0..n {
                let s1 = s[(i, j)];
                let k = s1 * ising2d_sum_of_adjacent_spins(&s, m, n, i, j);
                //let Open01(val) = random::<Open01<f64>>();
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
    let mut s = Array::<i64, _>::zeros(n * n).into_shape((n, n)).unwrap();
    for i in 0..n {
        for j in 0..n {
            let Open01(val) = random::<Open01<f32>>();
            if val < 0.5 {
                s[(i, j)] = -1;
            } else {
                s[(i, j)] = 1;
            }
        }
    }

    let niter = 1000_000_000;
    let beta = f64::ln(1.0 + f64::sqrt(2.0)) / 2.0;
    let start = Instant::now();
    ising2d_sweep(s, beta, niter);
    let end = start.elapsed();
    println!(
        "{}.{:03}秒経過しました。",
        end.as_secs(),
        end.subsec_nanos() / 1_000_000
    );
}
