use proconio::{fastout, input};
use std::collections::HashSet;

#[fastout]
fn main() {
    input! {
        a: usize, b: usize, c: usize, d: usize,
    };

    let mut cards = HashSet::new();
    for i in [a, b, c, d] {
        cards.insert(i);
    }

    if cards.len() == 2 {
        println!("Yes");
    } else {
        println!("No");
    }
}
