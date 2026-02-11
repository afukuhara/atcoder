use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize, k: usize,
        a: [usize; n],
    };

    let mut dp = vec![false; k + 1];

    for i in 1..=k {
        for j in 0..n {
            if (i as isize - a[j] as isize) >= 0 {
                dp[i] |= !dp[i - a[j]];
            }
        }
    }

    println!("{}", if dp[k] { "First" } else { "Second" });
}
