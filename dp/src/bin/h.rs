use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        h: usize, w: usize,
        matrix: [String; h],
    };

    let mut dp = vec![vec![0; w]; h];
    dp[0][0] = 1;
    let divider = 1_000__000_000 + 7;

    for (i, line) in matrix.iter().enumerate() {
        for (j, column) in line.chars().enumerate() {
            if column == '#' {
                continue;
            }

            if i != 0 {
                dp[i][j] += dp[i - 1][j];
            }
            if j != 0 {
                dp[i][j] += dp[i][j - 1];
            }

            dp[i][j] %= divider;
        }
    }

    println!("{:?}", dp[h - 1][w - 1]);
}


