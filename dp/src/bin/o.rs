use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        matrix: [[usize; n]; n],
    };

    const MOD: usize = 1_000_000_007;
    let pattern = 1 << n;
    let mut dp = vec![0; pattern];
    dp[0] = 1;

    for s in 0..(pattern - 1) {
        let i = s.count_ones() as usize;
        for j in 0..n {
            if matrix[i][j] == 1 && ((s >> j) & 1 == 0) {
                let next = s | (1 << j);
                dp[next] = (dp[next] + dp[s]) % MOD;
            }
        }
    }

    println!("{}", dp[pattern - 1]);
}
