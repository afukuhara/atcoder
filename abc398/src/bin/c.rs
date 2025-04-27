use proconio::{fastout, input};
use std::collections::{HashMap, HashSet};

#[fastout]
fn main() {
    input! {        n: usize   };

    let mut only_ones = HashSet::new();
    let mut more_than_two = HashSet::new();
    let mut people = HashMap::new();

    for i in 0..n {
        input! {
            a: usize,
        }

        if more_than_two.contains(&a) {
            continue;
        }

        if only_ones.contains(&a) {
            only_ones.remove(&a);
            more_than_two.insert(a);
            continue;
        }

        only_ones.insert(a);
        people.insert(a, i + 1);
    }

    let ans = only_ones.iter().max().unwrap_or(&0);

    if ans == &0 {
        println!("-1");
    } else {
        println!("{}", people[ans]);
    }
}
