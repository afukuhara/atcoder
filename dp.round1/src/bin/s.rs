use proconio::{fastout, input};

const MOD: usize = 1_000_000_007;

#[fastout]
fn main() {
    input! {
        k: String, d: usize
    };

    let str_len = k.len();
    let mut dp = vec![vec![vec![0; 2]; d]; str_len + 1];
    dp[0][0][1] = 1;

    for (i, c) in k
        .chars()
        .map(|c| c.to_digit(10).unwrap() as usize)
        .enumerate()
    {
        for r in 0..d {
            for tight in 0..2 {
                let cur = dp[i][r][tight];
                if cur == 0 {
                    continue;
                }

                let max_digit = if tight == 1 { c } else { 9 };

                for x in 0..=max_digit {
                    let new_r = (r + x) % d;
                    let new_tight = (tight == 1 && x == c) as usize;
                    dp[i + 1][new_r][new_tight] += cur;
                    dp[i + 1][new_r][new_tight] %= MOD;
                }
            }
        }
    }

    let ans = (dp[str_len][0][0] + dp[str_len][0][1] + MOD - 1) % MOD;
    println!("{}", ans);
}
