// thanks to κeen

extern crate ndarray;
extern crate sfmt;

pub mod torus;

use ndarray::LinalgScalar;

pub trait StencilArray {
    type Elem: LinalgScalar;
    fn stencil_map<Func>(&self, out: &mut Self, Func)
    where
        Func: Fn(Neigbhors<Self::Elem>) -> Self::Elem;
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
