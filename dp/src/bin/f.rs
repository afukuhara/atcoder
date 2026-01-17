use proconio::{fastout, input};
use std::cmp::max;

#[fastout]
fn main() {
    input! {
        s: String, t: String,
    };

    let s = s.chars().collect::<Vec<char>>();
    let t = t.chars().collect::<Vec<char>>();

    let mut dp = vec![vec![0; t.len() + 1]; s.len() + 1];
    for i in 0..s.len() {
        for j in 0..t.len() {
            if s[i] == t[j] {
                dp[i + 1][j + 1] = dp[i][j] + 1;
            }

            dp[i + 1][j + 1] = max(dp[i + 1][j + 1], dp[i + 1][j]);
            dp[i + 1][j + 1] = max(dp[i + 1][j + 1], dp[i][j + 1]);
        }
    }

    // 復元
    let mut ans = vec![];
    let mut i = s.len();
    let mut j = t.len();

    while i > 0 && j > 0 {
        if dp[i][j] == dp[i - 1][j] {
            i -= 1;
        } else if dp[i][j] == dp[i][j - 1] {
            j -= 1;
        } else {
            ans.push(s[i - 1]);
            i -= 1;
            j -= 1;
        }
    }

    println!("{}", ans.iter().rev().collect::<String>());
}
a