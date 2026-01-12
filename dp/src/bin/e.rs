use proconio::{fastout, input};
use std::cmp::min;

#[fastout]
fn main() {
    input! {
        n: usize, max_weight: usize,
        items: [(usize, usize); n],
    };

    let inf_weight = 10_000_000_000;
    let max_value = 100_000;
    let mut dp = vec![vec![inf_weight; max_value + 1]; n + 1];
    dp[0][0] = 0;

    for i in 1..=n {
        for v in 0..=max_value {
            let (weight, value) = items[i - 1];
            if v < value {
                dp[i][v] = dp[i - 1][v];
            } else {
                dp[i][v] = min(dp[i - 1][v], dp[i - 1][v - value] + weight);
            }
        }
    }

    let mut ans = 0;
    for (v, w) in dp[n].iter().enumerate() {
        if *w <= max_weight {
            ans = v;
        }
    }
    println!("{}", ans);
}
