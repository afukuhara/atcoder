use itertools::Itertools;
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        problem: String,
    };

    let (a, b) = problem
        .split("x")
        .map(|s| s.parse::<usize>().unwrap())
        .collect_tuple()
        .unwrap();
    println!("{}", a * b);
}
