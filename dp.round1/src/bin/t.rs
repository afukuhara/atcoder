use proconio::{fastout, input};

const MOD: usize = 1_000_000_007;

#[fastout]
fn main() {
    input! {
        n: usize, s: String
    };

    let s = s.chars().collect::<Vec<char>>();

    let mut dp = vec![vec![0; n + 1]; n + 1];
    dp[1][0] = 1;

    for i in 1..n {
        if s[i - 1] == '<' {
            let mut sumv = 0;
            for j in 0..=i {
                dp[i + 1][j] = sumv;
                sumv += dp[i][j];
                sumv %= MOD;
            }
        } else {
            let mut sumv = 0;
            for j in (0..i).rev() {
                sumv += dp[i][j];
                sumv %= MOD;
                dp[i + 1][j] = sumv;
            }
        }
    }

    let ans = dp[n][0..n].iter().sum::<usize>() % MOD;
    println!("{}", ans);
}
