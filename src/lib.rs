
use std::vec;

use pyo3::{prelude::*, types::PyList};

#[pyclass]
pub struct SegmentTree {
    n: usize,
    data: Vec<i64>,
}

#[pymethods]
impl SegmentTree {
    #[new]
    //#[pyo3(signature = (*arr))]
    pub fn new( arr: Vec<i64>) -> PyResult<Self> {
        let n = arr.len();
        let mut seg_tree = SegmentTree {
            n,
            data: vec![0; 4 * n],
        };
        seg_tree.build(1, 0, n - 1, arr);
        Ok(seg_tree)
    }

    fn build(&mut self, k: usize, l: usize, r: usize, arr: Vec<i64>) {
        if l == r {
            self.data[k] = arr[l];
        } else {
            let mid = (l + r) / 2;
            self.build(2 * k, l, mid, arr.clone());
            self.build(2 * k + 1, mid + 1, r, arr);
            self.data[k] = self.funcion(self.data[2 * k], self.data[2 * k + 1]);
        }
    }

    pub fn update(&mut self, mut i: usize, x: i64) {
        i += self.n;
        self.data[i] = x;
        while i > 1 {
            i /= 2;
            self.data[i] = self.funcion(self.data[2 * i], self.data[2 * i + 1]);
        }
    }

    pub fn query(&self, a: usize, b: usize) -> PyResult<i64> {
        Ok(self.query_(1, 0, self.n - 1, a, b))
    }

    fn query_(&self, k: usize, l: usize, r: usize, a: usize, b: usize) -> i64{
        if a > b {
            return self.neutro_element();
        }
        if a == l && r == b {
            return self.data[k];
        }
        let mid = (l + r) / 2;
        let left_res = self.query_(k * 2, l, mid, a, std::cmp::min(b, mid));
        let right_res = self.query_(k * 2 + 1, mid + 1, r, std::cmp::max(a, mid + 1), b);
        self.funcion(left_res, right_res)
    }

    fn funcion(&self, a: i64, b: i64) -> i64 {
        a + b
    }

    fn neutro_element(&self) -> i64 {
        0
    }
}

#[pymodule]
#[pyo3(name = "segment_tree")]
fn segment_tree(m: &Bound<'_,PyModule>) -> PyResult<()> {
    m.add_class::<SegmentTree>()?;
    Ok(())
}