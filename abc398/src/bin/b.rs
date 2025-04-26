use itertools::Itertools;
use proconio::{fastout, input};
use std::collections::HashMap;

#[fastout]
fn main() {
    let mut cards = HashMap::new();

    for _ in 0..7 {
        input! {
            c: usize,
        };

        let count = cards.entry(c).or_insert(0);
        *count += 1;
    }

    let answer = match cards.values().sorted().rev().take(2).collect_tuple() {
        Some((top, second)) if top >= &3 && second >= &2 => "Yes",
        _ => "No",
    };

    println!("{}", answer);
}
