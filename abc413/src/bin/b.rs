use itertools::Itertools;
use proconio::{fastout, input};
use std::collections::HashSet;

#[fastout]
fn main() {
    input! {
        n: usize,
        s: [String; n]
    };

    let mut answer = HashSet::new();
    (0..n).combinations(2).for_each(|v| {
        let a = v[0];
        let b = v[1];
        answer.insert(s[a].clone() + &s[b]);
        answer.insert(s[b].clone() + &s[a]);
    });

    println!("{}", answer.len());
}
