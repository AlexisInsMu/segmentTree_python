use std::cmp::{max, min};

pub struct SegmentTree{
    n: usize,
    data: Vec<i64>,
}

impl SegmentTree{
    pub fn new(n: usize, arr: &[i64]) -> Self{
        let mut seg_tree = SegmentTree {
            n,
            data: vec![0; 4 * n],
        };
        seg_tree.build(1, 0, n-1, arr);
        seg_tree
    }
    fn build(&mut self, k: usize, l: usize, r: usize, arr: &[i64]){
        if r == l {
            self.data[k] = arr[l];
        }else{
            let mid = (l+r)/2;
            self.build(2*k, l, mid, arr);
            self.build(2*k+1, mid+1, r, arr);
            self.data[k] = self.funcion(self.data[2*k], self.data[2*k+1]);
        }
    }

    pub fn update(&mut self, mut i: usize, x: i64){
        i += self.n;
        self.data[i] = x;
        while i > 1{
            i /= 2;
            self.data[i] = self.funcion(self.data[2*i], self.data[2*i+1]);
        }
    }
    pub fn  query(&self, a: usize, b: usize) -> i64{
        self.query_(1, 0, self.n-1, a, b)
    }
    fn query_(&self, k: usize, l: usize, r: usize, a: usize, b: usize) -> i64{
        if a>b {
            return self.neutro_element();
        }
        if a == l && r == b {
            return self.data[k];
        }
        let mid = (l+r)/2;
        let left_res = self.query_(
            k.checked_mul(2).expect("Multiplication overflow"),
            l,
            mid,
            a,
            min(b, mid),
        );
        let right_res = self.query_(
            k.checked_mul(2).expect("Multiplication overflow")
                .checked_add(1).expect("Addition overflow"),
            mid + 1,
            r,
            max(a, mid + 1),
            b,
        );
        self.funcion(left_res, right_res)
    }
    fn funcion(&self, a: i64, b: i64) -> i64{
        a + b
    }

    fn neutro_element(&self) -> i64 {
        0
    }
}

fn main() {
        let arr = vec![1, 2, 3, 4, 5];
        let mut seg_tree = SegmentTree::new(arr.len(), &arr);
        println!("{:?}", seg_tree.query(0, 4)); // Output: 15
        println!("{:?}", seg_tree.query(2, 4)); // Output: 12
        println!("{:?}", seg_tree.data); // Output: 6
        seg_tree.update(2, 10);
        println!("{:?}", seg_tree.query(0, 4)); // Output: 22
        println!("{:?}", seg_tree.data); // Output: 19

}
