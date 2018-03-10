extern crate ising;
#[macro_use]
extern crate timeit;

use std::mem;
use ising::{step, StencilArray, torus::Torus2};

fn ising2d<Arr>(mut s: Arr, beta: f32, iter: usize)
where
    Arr: StencilArray<Elem = i8> + Clone,
{
    let st = step(beta);
    let mut s2 = s.clone();
    for _ in 0..iter {
        s.stencil_map(&mut s2, &st);
        mem::swap(&mut s, &mut s2);
    }
}

fn main() {
    let n = 100;
    let iter = 100_000;
    let beta = (1.0 + 2.0_f32.sqrt()).ln() / 2.0;
    timeit!({
        let s = Torus2::random_spin(n, n);
        ising2d(s, beta, iter);
    });
}
