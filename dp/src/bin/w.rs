use proconio::{fastout, input};
use std::cmp::max;

#[fastout]
fn main() {
    input! {
        n: usize, m: usize,
        scores: [(usize, usize, i64); m]
    };

    let mut events = vec![vec![]; n + 1];
    for (a, b, c) in scores {
        events[b].push((a, c));
    }

    let neg_inf = i64::MIN;
    let mut seg = LazySegTree::new(n + 1, neg_inf);
    seg.set(0, 0);

    for i in 1..=n {
        let best = seg.query(0, i);
        seg.set(i, best);

        for &(l, a) in &events[i] {
            seg.add(l, i + 1, a);
        }
    }

    println!("{}", seg.query(0, n + 1));
}

struct LazySegTree {
    size: usize,
    data: Vec<i64>,
    lazy: Vec<i64>,
    neg_inf: i64,
}

impl LazySegTree {
    fn new(n: usize, neg_inf: i64) -> Self {
        let mut size = 1;
        while size < n {
            size *= 2;
        }
        Self {
            size,
            data: vec![neg_inf; 2 * size],
            lazy: vec![0; 2 * size],
            neg_inf,
        }
    }

    fn apply(&mut self, k: usize, x: i64) {
        self.data[k] += x;
        self.lazy[k] += x;
    }

    fn push(&mut self, k: usize) {
        if self.lazy[k] != 0 {
            let x = self.lazy[k];
            self.apply(k * 2, x);
            self.apply(k * 2 + 1, x);
            self.lazy[k] = 0;
        }
    }

    fn pull(&mut self, k: usize) {
        self.data[k] = max(self.data[k * 2], self.data[k * 2 + 1]);
    }

    fn set(&mut self, index: usize, value: i64) {
        self.set_rec(index, value, 1, 0, self.size);
    }

    fn set_rec(&mut self, index: usize, value: i64, k: usize, left: usize, right: usize) {
        if right - left == 1 {
            self.data[k] = value;
            self.lazy[k] = 0;
            return;
        }

        self.push(k);

        let middle = (left + right) / 2;

        if index < middle {
            self.set_rec(index, value, k * 2, left, middle);
        } else {
            self.set_rec(index, value, k * 2 + 1, middle, right);
        }

        self.pull(k);
    }

    fn add(&mut self, l: usize, r: usize, x: i64) {
        self.add_rec(l, r, x, 1, 0, self.size);
    }

    fn add_rec(&mut self, l: usize, r: usize, x: i64, k: usize, left: usize, right: usize) {
        if right <= l || r <= left {
            return;
        }

        if l <= left && right <= r {
            self.apply(k, x);
            return;
        }

        self.push(k);

        let middle = (left + right) / 2;

        self.add_rec(l, r, x, k * 2, left, middle);
        self.add_rec(l, r, x, k * 2 + 1, middle, right);

        self.pull(k);
    }

    fn query(&mut self, l: usize, r: usize) -> i64 {
        self.query_rec(l, r, 1, 0, self.size)
    }

    fn query_rec(&mut self, l: usize, r: usize, k: usize, left: usize, right: usize) -> i64 {
        if right <= l || r <= left {
            return self.neg_inf;
        }

        if l <= left && right <= r {
            return self.data[k];
        }

        self.push(k);

        let middle = (left + right) / 2;
        let left_val = self.query_rec(l, r, k * 2, left, middle);
        let right_val = self.query_rec(l, r, k * 2 + 1, middle, right);

        max(left_val, right_val)
    }
}
