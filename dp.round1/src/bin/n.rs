use proconio::{fastout, input};
use std::cmp::min;

#[fastout]
fn main() {
    input! {
        n: usize,
        slimes: [u64; n]
    };

    let mut sum_dp = vec![0; n + 1];
    for i in 0..n {
        sum_dp[i + 1] = sum_dp[i] + slimes[i];
    }

    let mut dp = vec![vec![u64::MAX; n + 1]; n + 1];
    for i in 0..n {
        dp[i][i] = 0;
    }

    let ans = rec(0, n - 1, &mut dp, &sum_dp);
    println!("{}", ans);
}

fn rec(l: usize, r: usize, dp: &mut Vec<Vec<u64>>, sum_dp: &[u64]) -> u64 {
    if dp[l][r] != u64::MAX {
        return dp[l][r];
    }

    for m in l..r {
        let cost = rec(l, m, dp, sum_dp) + rec(m + 1, r, dp, sum_dp) + (sum_dp[r + 1] - sum_dp[l]);
        dp[l][r] = min(dp[l][r], cost);
    }

    dp[l][r]
}
