use itertools::Itertools;
use proconio::{fastout, input};
use std::collections::HashSet;

#[fastout]
fn main() {
    input! {
        n: usize, m: usize,
    };

    let mut a_set = HashSet::new();
    for _ in 0..m {
        input! {
            a: usize,
        }

        a_set.insert(a);
    }

    let mut ans = vec![];
    for i in 1..=n {
        if !a_set.contains(&i) {
            ans.push(i);
        }
    }

    println!("{}", ans.len());
    println!("{}", ans.iter().join(" "));
}
