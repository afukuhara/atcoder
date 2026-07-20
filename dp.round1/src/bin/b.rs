use proconio::{fastout, input};
use std::cmp::min;

#[fastout]
fn main() {
    input! {
        n: usize,
        k: usize,
        h: [i32; n]
    };

    let mut dp = vec![0; n];
    dp[0] = 0;
    for i in 1..n {
        dp[i] = (1..=min(i, k))
            .map(|j| dp[i - j] + (h[i] - h[i - j]).abs())
            .min()
            .unwrap();
    }

    println!("{}", dp[n - 1]);
}
