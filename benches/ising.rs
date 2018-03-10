#![feature(test)]

extern crate ising;
extern crate test;

use test::*;
use ising::{step, StencilArray, torus::Torus2};

#[bench]
fn ising(b: &mut Bencher) {
    let n = 100;
    let beta = (1.0 + 2.0_f32.sqrt()).ln() / 2.0;
    let st = step(beta);
    let s = Torus2::random_spin(n, n);
    let mut s2 = s.clone();
    b.iter(|| {
        s.stencil_map(&mut s2, &st);
    });
}
