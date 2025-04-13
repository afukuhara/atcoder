use proconio::{fastout, input};
use std::cmp::max;
use std::collections::HashSet;

#[fastout]
fn main() {
    input! {
        n: usize,
        nums: [usize; n],
    };

    let mut left_set = HashSet::new();
    let mut right_set = HashSet::new();

    let mut left = vec![0; n];
    let mut right = vec![0; n];

    for (i, num) in nums.iter().enumerate() {
        left_set.insert(num);
        left[i] = left_set.len();
    }

    for (i, num) in nums.iter().rev().enumerate() {
        right_set.insert(num);
        right[i] = right_set.len();
    }

    let mut ans = 0;
    for i in 0..n - 1 {
        ans = max(ans, left[i] + right[n - i - 2]);
    }

    println!("{}", ans);
}
