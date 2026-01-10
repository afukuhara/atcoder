use proconio::{fastout, input};
use std::cmp::max;

#[fastout]
fn main() {
    input! {
        n: usize,
        activities: [usize; 3],
    };

    let mut dp = vec![vec![0; 3]; n];
    dp[0][0] = activities[0];
    dp[0][1] = activities[1];
    dp[0][2] = activities[2];

    for i in 1..n {
        input! {
            activities: [usize; 3],
        };
        dp[i][0] = max(dp[i - 1][1], dp[i - 1][2]) + activities[0];
        dp[i][1] = max(dp[i - 1][0], dp[i - 1][2]) + activities[1];
        dp[i][2] = max(dp[i - 1][0], dp[i - 1][1]) + activities[2];
    }

    println!("{}", dp[n - 1].iter().max().unwrap());
}
