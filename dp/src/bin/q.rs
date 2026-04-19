use proconio::{fastout, input, marker::Usize1};
use std::cmp::max;

#[fastout]
fn main() {
    input! {
        n: usize,
        h: [Usize1; n],
        a: [usize; n],
    };

    let mut seg_tree = SegTree::new(n);

    for i in 0..n {
        let val = seg_tree.query(0, h[i]);
        let new_val = val + a[i] as i64;
        seg_tree.update(h[i], new_val);
    }

    println!("{}", seg_tree.query(0, n));
}

struct SegTree {
    n: usize,
    data: Vec<i64>,
}

impl SegTree {
    fn new(size: usize) -> Self {
        let mut n = 1;
        while n < size {
            n *= 2;
        }
        Self {
            n,
            data: vec![0; 2 * n],
        }
    }

    fn update(&mut self, mut i: usize, x: i64) {
        i += self.n;
        self.data[i] = x;
        while i > 1 {
            i /= 2;
            self.data[i] = max(self.data[i * 2], self.data[i * 2 + 1]);
        }
    }

    fn query(&self, mut l: usize, mut r: usize) -> i64 {
        let mut res = 0;
        l += self.n;
        r += self.n;
        while l < r {
            if l & 1 == 1 {
                res = max(res, self.data[l]);
                l += 1;
            }
            if r & 1 == 1 {
                r -= 1;
                res = max(res, self.data[r]);
            }
            l >>= 1;
            r >>= 1;
        }
        res
    }
}
