use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        a: [usize; n],
    };

    let mut dp = vec![vec![vec![-1.0; n + 1]; n + 1]; n + 1];
    let mut one = 0;
    let mut two = 0;
    let mut three = 0;

    for i in a {
        match i {
            1 => one += 1,
            2 => two += 1,
            3 => three += 1,
            _ => unreachable!(),
        }
    }

    let ans = rec(one, two, three, &mut dp, n);
    println!("{}", ans);
}

fn rec(i: usize, j: usize, k: usize, dp: &mut [Vec<Vec<f64>>], len: usize) -> f64 {
    if dp[i][j][k] >= 0.0 {
        return dp[i][j][k];
    }

    if i == 0 && j == 0 && k == 0 {
        return 0.0;
    }

    let mut res = 0.0;
    if i > 0 {
        res += rec(i - 1, j, k, dp, len) * i as f64;
    }
    if j > 0 {
        res += rec(i + 1, j - 1, k, dp, len) * j as f64;
    }
    if k > 0 {
        res += rec(i, j + 1, k - 1, dp, len) * k as f64;
    }

    res += len as f64;
    res *= 1.0 / (i + j + k) as f64;

    dp[i][j][k] = res;
    res
}
