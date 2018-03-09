// thanks to κeen

extern crate ndarray;
extern crate rand;
extern crate sfmt;
#[macro_use]
extern crate timeit;

use ndarray::*;

use rand::{random, Rng};
use sfmt::SFMT;

struct PadArray2<A: LinalgScalar> {
    data: Array2<A>,
}

struct Neigbhors<A> {
    t: A, // top
    b: A, // bottom
    l: A, // left
    r: A, // right
    c: A, // center
}

impl<A: LinalgScalar> PadArray2<A> {
    fn zeros(n: usize, m: usize) -> Self {
        Self {
            data: Array::zeros((n + 2, m + 2)),
        }
    }

    fn shape(&self) -> (usize, usize) {
        let (n, m) = self.data.dim();
        (n - 2, m - 2)
    }

    fn fill_periodic(&mut self) {
        let (n, m) = self.shape();
        for j in 0..m {
            self.data[(0, j + 1)] = self.data[(n - 2, j + 1)];
            self.data[(n - 1, j + 1)] = self.data[(1, j + 1)];
        }
        for i in 0..n {
            self.data[(i + 1, 0)] = self.data[(i + 1, m - 2)];
            self.data[(i + 1, m - 1)] = self.data[(i + 1, 1)];
        }
    }

    fn st_map<B, F>(&self, out: &mut PadArray2<B>, func: F)
    where
        B: LinalgScalar,
        F: Fn(Neigbhors<A>) -> B,
    {
        let (n, m) = self.shape();
        for i in 0..n {
            for j in 0..m {
                let neighbor = Neigbhors {
                    t: self.data[(i + 0, j + 0)],
                    b: self.data[(i + 2, j + 0)],
                    l: self.data[(i + 1, j + 0)],
                    r: self.data[(i + 1, j + 2)],
                    c: self.data[(i + 1, j + 1)],
                };
                out.data[(i + 1, j + 1)] = func(neighbor);
            }
        }
    }
}

#[inline]
fn ising2d_sum_of_adjacent_spins(s: &Array2<i8>, m: usize, n: usize, i: usize, j: usize) -> i8 {
    let i_bottom = if i + 1 < m { i + 1 } else { 0 };
    let i_top = if i >= 1 { i - 1 } else { m - 1 };
    let j_right = if j + 1 < n { j + 1 } else { 0 };
    let j_left = if j >= 1 { j - 1 } else { n - 1 };
    return s[(i_bottom, j)] + s[(i_top, j)] + s[(i, j_right)] + s[(i, j_left)];
}

fn ising2d_sweep(mut s: Array2<i8>, beta: f32, niter: usize) {
    let m = s.shape()[0];
    let n = s.shape()[1];
    let p = prob(beta);
    let mut rng = SFMT::new(1234);
    let iteration = niter / (m * n);
    for _ in 0..iteration {
        for i in 0..m {
            for j in 0..n {
                let s1 = s[(i, j)];
                let k = s1 * ising2d_sum_of_adjacent_spins(&s, m, n, i, j);
                let val = rng.gen_range(0., 1.);
                if val < p[(k + 4) as usize] {
                    s[(i, j)] = -s1;
                }
            }
        }
    }
}

fn prob(beta: f32) -> Vec<f32> {
    (-4..5).map(|s| (-2.0 * beta * s as f32).exp()).collect()
}

fn init(n: usize, m: usize) -> Array2<i8> {
    let mut s = Array::<i8, _>::zeros((n, m));
    for i in 0..n {
        for j in 0..m {
            let val: f32 = random();
            s[(i, j)] = if val < 0.5 { -1 } else { 1 };
        }
    }
    s
}

fn main() {
    let n = 100;
    let niter = 1000_000_000;
    let beta = (1.0 + 2.0_f32.sqrt()).ln() / 2.0;
    timeit!({
        let s = init(n, n);
        ising2d_sweep(s, beta, niter);
    });
}
