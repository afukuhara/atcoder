use proconio::{fastout, input};
use std::collections::BTreeSet;

#[fastout]
fn main() {
    input! {
        t: usize
    };

    for _ in 0..t {
        input! {
            n: usize, case: [usize; n]
        }

        let mut current_weight = case[0];
        let end = case[n - 1];

        let dominos = case[1..n - 1].iter().collect::<BTreeSet<_>>();

        let mut count = 2;
        while current_weight * 2 < end {
            if let Some(next_weight) = dominos.range(current_weight + 1..=current_weight * 2).max()
            {
                current_weight = **next_weight;
                count += 1;
            } else {
                count = -1;
                break;
            }
        }
        println!("{}", count);
    }
}
