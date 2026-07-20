use proconio::{fastout, input};
use std::cmp::max;

#[fastout]
fn main() {
    input! {
        n: usize,
        plan: [[usize; 3]; n]
    };

    let mut dp = vec![vec![0, 0, 0]; n];
    dp[0] = plan[0].clone();

    for i in 1..n {
        dp[i][0] = max(dp[i - 1][1], dp[i - 1][2]) + plan[i][0];
        dp[i][1] = max(dp[i - 1][0], dp[i - 1][2]) + plan[i][1];
        dp[i][2] = max(dp[i - 1][0], dp[i - 1][1]) + plan[i][2];
    }

    println!("{:?}", dp[n - 1].iter().max().unwrap());
}
