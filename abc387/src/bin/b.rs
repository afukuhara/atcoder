#[allow(unused_imports)]
use itertools::{iproduct, Itertools};
#[allow(unused_imports)]
use num_traits::pow;
#[allow(unused_imports)]
use proconio::{
    fastout, input,
    marker::{Chars, Usize1},
};
#[allow(unused_imports)]
use std::cmp::{max, min};
#[allow(unused_imports)]
use std::collections::{HashMap, HashSet, VecDeque};
#[allow(unused_imports)]
use std::iter::FromIterator;

#[fastout]
fn main() {
    input! {
        x: usize
    };

    let mut numbers = vec![];
    for x in 1..=9 {
        for y in 1..=9 {
            numbers.push(x * y);
        }
    }

    println!("{}", numbers.iter().filter(|&&n| n != x).sum::<usize>());
}
