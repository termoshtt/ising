// thanks to κeen
#![feature(conservative_impl_trait)]

#[macro_use]
extern crate ndarray;
extern crate rand;
extern crate sfmt;

pub mod torus;

use rand::Rng;
use ndarray::*;

pub trait StencilArray {
    type Elem: LinalgScalar;
    fn stencil_map<Func>(&self, out: &mut Self, Func)
    where
        Func: Fn(Neigbhors<Self::Elem>) -> Self::Elem;

    fn shape(&self) -> (usize, usize);
}

pub trait Viewable {
    type Elem: LinalgScalar;
    type Dim: Dimension;
    fn as_view(&self) -> ArrayView<Self::Elem, Self::Dim>;
    fn as_view_mut(&mut self) -> ArrayViewMut<Self::Elem, Self::Dim>;
}

#[derive(Clone, Copy)]
pub struct Neigbhors<A: Clone + Copy> {
    pub t: A, // top
    pub b: A, // bottom
    pub l: A, // left
    pub r: A, // right
    pub c: A, // center
}

/// Transition probability for Ising model
pub fn trans_prob(beta: f32) -> Vec<f32> {
    (-4..5).map(|s| (-2.0 * beta * s as f32).exp()).collect()
}

/// Iteration step of ising model for stencil_map
pub fn step(beta: f32) -> impl Fn(Neigbhors<i8>) -> i8 {
    let p = trans_prob(beta);
    move |n| {
        let k = n.t + n.b + n.l + n.r + 4;
        let val: f32 = sfmt::thread_rng().gen();
        if val < p[k as usize] {
            -n.c
        } else {
            n.c
        }
    }
}
