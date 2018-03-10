extern crate ising;
extern crate ndarray;
extern crate rand;
extern crate sfmt;
#[macro_use]
extern crate timeit;

use std::mem;
use rand::Rng;
use ising::{StencilArray, torus::Torus2};

fn ising2d<Arr>(mut s: Arr, beta: f32, iter: usize)
where
    Arr: StencilArray<Elem = i8> + Clone,
{
    let p = ising::trans_prob(beta);
    let mut s2 = s.clone();
    for _ in 0..iter {
        s.stencil_map(&mut s2, |n| {
            let k = n.t + n.b + n.l + n.r + 4;
            let val: f32 = sfmt::thread_rng().gen();
            if val < p[k as usize] {
                -n.c
            } else {
                n.c
            }
        });
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
