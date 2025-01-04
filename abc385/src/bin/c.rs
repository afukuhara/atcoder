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
    let n = input.trim_end().parse::<usize>().unwrap();

    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let building_heights: Vec<usize> = input
        .split_ascii_whitespace()
        .map(|s| s.parse::<usize>().unwrap())
        .collect();

    let mut answer = 1;
    for (i, height) in building_heights.iter().enumerate() {
        for j in i + 1..n {
            if height == &building_heights[j] {
                let mut count = 2;
                let step = j - i;
                for k in (j + step..n).step_by(step) {
                    if height == &building_heights[k] {
                        count += 1;
                    } else {
                        break;
                    }
                }
                answer = std::cmp::max(answer, count);
            }
        }
    }
    println!("{}", answer);
}
