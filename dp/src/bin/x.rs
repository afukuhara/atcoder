use itertools::Itertools;
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        blocks: [(usize, usize, usize); n]
    };

    let limit = 20000;

    let mut dp = vec![(0, 0); limit + 1];
    for (w, s, v) in blocks
        .iter()
        .sorted_by_key(|(weight, strength, _)| *weight + *strength)
    {
        for weight in (0..=limit).rev() {
            let (_, value) = dp[weight];
            let new_weight = weight + *w;
            let new_value = value + *v;
            if *s >= weight && dp[new_weight].1 < new_value {
                dp[new_weight] = (*s, new_value);
            }
        }
    }

    println!("{}", dp.iter().map(|(_, value)| value).max().unwrap());
}
