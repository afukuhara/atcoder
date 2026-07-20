use proconio::{fastout, input};
use std::cmp::min;

#[fastout]
fn main() {
    input! {
        n: usize, k: usize,
        h: [isize; n],
    };

    let mut dp = vec![isize::MAX; n];

    dp[0] = 0;

    for i in 1..n {
        for j in 1..=min(k, i) {
            dp[i] = min(dp[i], dp[i - j] + (h[i] - h[i - j]).abs());
        }
    }

    println!("{:?}", dp[n - 1]);
}
