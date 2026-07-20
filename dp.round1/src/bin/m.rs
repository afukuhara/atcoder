#[allow(unused_imports)]
use itertools::{iproduct, Itertools};
#[allow(unused_imports)]
use num_traits::pow;
#[allow(unused_imports)]
use proconio::{
    fastout, input,
    marker::{Chars, Usize1},
};
#[allow(unused_imports)]
use std::cmp::{max, min};
#[allow(unused_imports)]
use std::collections::{HashMap, HashSet, VecDeque};
#[allow(unused_imports)]
use std::iter::FromIterator;

#[fastout]
fn main() {
    input! {
        n: usize, k: usize,
        a: [usize; n],
    };

    const MOD: i64 = 1_000_000_007;
    let mut dp = vec![vec![0i64; k + 1]; n + 1];
    dp[0][0] = 1;

    for i in 0..n {
        // dp[i] の累積和を求める
        let mut sum_dp = vec![0i64; k + 2];
        for j in 0..=k {
            sum_dp[j + 1] = (sum_dp[j] + dp[i][j]) % MOD;
        }

        // dp[i+1] を求める
        for j in 0..=k {
            let left = j - min(j, a[i]);
            let val = sum_dp[j + 1] - sum_dp[left];
            dp[i + 1][j] = (val % MOD + MOD) % MOD; // 負対策
        }
    }

    println!("{}", dp[n][k] % MOD);
}
