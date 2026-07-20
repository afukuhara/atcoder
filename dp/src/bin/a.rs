use num::abs;
use proconio::{fastout, input};
use std::cmp::min;

#[fastout]
fn main() {
    input! {
        n: usize,
        h: [isize; n],
    };

    let mut dp = vec![0; n];

    dp[0] = 0;
    dp[1] = abs(h[1] - h[0]);

    for i in 2..n {
        let a = abs(h[i] - h[i - 1]);
        let b = abs(h[i] - h[i - 2]);
        dp[i] = min(dp[i - 1] + a, dp[i - 2] + b);
    }

    println!("{:?}", dp[n - 1]);
}
