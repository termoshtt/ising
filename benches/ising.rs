#![feature(test)]

extern crate ising;
extern crate rand;
extern crate sfmt;
extern crate test;

use test::*;
use rand::Rng;
use ising::{trans_prob, StencilArray, torus::Torus2};

#[bench]
fn ising(b: &mut Bencher) {
    let n = 100;
    let beta = (1.0 + 2.0_f32.sqrt()).ln() / 2.0;
    let p = trans_prob(beta);
    let s = Torus2::random_spin(n, n);
    let mut s2 = s.clone();
    b.iter(|| {
        s.stencil_map(&mut s2, |n| {
            let k = n.t + n.b + n.l + n.r + 4;
            let val: f32 = sfmt::thread_rng().gen();
            if val < p[k as usize] {
                -n.c
            } else {
                n.c
            }
        });
    });
}
