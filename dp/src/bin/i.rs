use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        mut probs: [f64; n]
    };

    let max_back = (n / 2) + 1;
    let initial_face = probs[0];
    let initial_back = 1.0 - probs[0];

    let mut dp = vec![vec![0.0; max_back]; n];
    dp[0][0] = initial_face;

    if max_back > 1 {
        dp[0][1] = initial_back;
    }

    for i in 1..n {
        let face = probs[i];
        let back = 1.0 - probs[i];

        dp[i][0] = dp[i - 1][0] * face;
        for j in 1..max_back {
            dp[i][j] = dp[i - 1][j - 1] * back + dp[i - 1][j] * face;
        }
    }

    println!("{}", dp[n - 1].iter().sum::<f64>());
}
b t