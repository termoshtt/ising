use ndarray::*;

#[derive(Clone)]
pub struct PadArray2<A: LinalgScalar> {
    data: Array2<A>,
}

pub struct Neigbhors<A> {
    pub t: A, // top
    pub b: A, // bottom
    pub l: A, // left
    pub r: A, // right
    pub c: A, // center
}

impl<A: LinalgScalar> PadArray2<A> {
    pub fn zeros(n: usize, m: usize) -> Self {
        Self {
            data: Array::zeros((n + 2, m + 2)),
        }
    }

    pub fn from_map<F>(n: usize, m: usize, f: F) -> Self
    where
        F: Fn(usize, usize) -> A,
    {
        let mut z = Self::zeros(n, m);
        for i in 0..n {
            for j in 0..m {
                z.data[(i + 1, j + 1)] = f(i, j);
            }
        }
        z
    }

    pub fn shape(&self) -> (usize, usize) {
        let (n, m) = self.data.dim();
        (n - 2, m - 2)
    }

    #[inline(never)]
    pub fn fill_periodic(&mut self) {
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

    #[inline(never)]
    pub fn st_map<B, F>(&self, out: &mut PadArray2<B>, func: F)
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
