use itertools::Itertools;
use proconio::{fastout, input};
use std::cmp::max;

#[fastout]
fn main() {
    input! {
        n: usize
    };

    let mut a = vec![];
    for _ in 0..n {
        input! {
            a_in: [i128; n]
        };
        a.push(a_in);
    }

    let mut score = vec![0; 1 << n];

    for i in 1..(1 << n) {
        let mut members = vec![];
        let mut j = 0;
        let mut k = i;
        while k > 0 {
            if k & 1 == 1 {
                members.push(j);
            }
            j += 1;
            k >>= 1;
        }

        for pair in members
            .iter()
            .combinations(2)
            .filter(|pair| pair[0] < pair[1])
        {
            let (ni, nj) = (*pair[0], *pair[1]);
            score[i] += a[ni][nj];
        }
    }

    let size = 1 << n;
    let mut dp = vec![std::i128::MIN; size];

    dp[0] = 0;

    for mask in 1..size {
        let first = mask & mask.wrapping_neg();

        let mut sub = mask;
        while sub > 0 {
            if (sub & first) != 0 {
                dp[mask] = max(dp[mask], dp[mask ^ sub] + score[sub]);
            }

            sub = (sub - 1) & mask;
        }
    }

    println!("{}", dp[size - 1]);
}
