use proconio::{fastout, input};
use std::cmp::max;

#[fastout]
fn main() {
    input! {
        n: usize, max_weight: usize,
        items: [(usize, u128); n],
    };

    let mut dp = vec![vec![0; max_weight + 1]; n + 1];
    for i in 1..=n {
        for w in 1..=max_weight {
            let (weight, value) = items[i - 1];
            if w < weight {
                dp[i][w] = dp[i - 1][w];
            } else {
                dp[i][w] = max(dp[i - 1][w], dp[i - 1][w - weight] + value);
            }
        }
    }

    println!("{}", dp[n][max_weight]);
}
