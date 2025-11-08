use itertools::Itertools;
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize, m: usize,
        mut x: [usize; n],
    };

    x.sort();
    let total = x
        .windows(2)
        .map(|w| w[1] - w[0])
        .sorted()
        .take(n - m)
        .sum::<usize>();

    println!("{}", total);
}
