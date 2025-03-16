use proconio::{fastout, input};
use std::cmp::{max, min};
use std::collections::HashSet;

#[fastout]
fn main() {
    input! {
        _n: usize, m: usize,
    };

    let mut count = 0;
    let mut graph = HashSet::new();
    for _ in 0..m {
        input! {
            u: usize, v: usize,
        }

        let branch = (min(u, v), max(u, v));
        if graph.contains(&branch) || u == v {
            count += 1;
        }

        graph.insert(branch);
    }

    println!("{}", count);
}
