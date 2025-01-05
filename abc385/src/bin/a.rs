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
pub fn main() {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();

    let nums: Vec<i32> = input
        .split_ascii_whitespace()
        .map(|s| s.parse::<i32>().unwrap())
        .collect();
    let [a, b, c] = nums[..] else {
        panic!("Expected 2 numbers")
    };

    if a == b && b == c {
        println!("Yes");
    } else if a + b == c || a + c == b || b + c == a {
        println!("Yes");
    } else {
        println!("No");
    }
}
