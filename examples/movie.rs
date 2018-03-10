extern crate image;
extern crate ising;

use std::{fs, mem};
use ising::{step, StencilArray, Viewable, torus::Torus2};

fn ising2d<Arr>(mut s: Arr, beta: f32, iter: usize)
where
    Arr: StencilArray<Elem = i8> + Viewable<Elem = i8> + Clone,
{
    let st = step(beta);
    let mut s2 = s.clone();
    for i in 0..iter {
        s.stencil_map(&mut s2, &st);
        to_png(i, &s);
        mem::swap(&mut s, &mut s2);
    }
}

fn to_png<Arr>(i: usize, s: &Arr)
where
    Arr: StencilArray<Elem = i8> + Viewable<Elem = i8>,
{
    use image::*;
    let raw: Vec<_> = s.as_view().iter().map(|&v| v as u8).collect();
    let (n, m) = s.shape();
    let buf = ImageBuffer::<Luma<_>, _>::from_raw(n as u32, m as u32, raw).unwrap();
    let fname = format!("movie/ising_{:05}.png", i);
    buf.save(&fname).unwrap();
    eprintln!("Generate PNG: {}", fname);
}

fn main() {
    let n = 100;
    let iter = 1000;
    let beta = (1.0 + 2.0_f32.sqrt()).ln() / 2.0;
    let s = Torus2::random_spin(n, n);
    fs::create_dir_all("movie").unwrap();
    ising2d(s, beta, iter);
}
