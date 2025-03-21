use proconio::{fastout, input};
use std::cmp::min;

#[fastout]
fn main() {
    input! {
        _n: usize
    };

    input! {
        s: String
    };

    let s: Vec<char> = s.chars().collect();
    let total_block_count = s.iter().filter(|&c| *c == '1').count();

    let mut now = 0;
    let mut answer = 0;

    for char in s.iter() {
        match char {
            '0' => {
                answer += min(now, total_block_count - now);
            }
            '1' => {
                now += 1;
            }
            _ => unreachable!(),
        }
    }

    println!("{}", answer);
}
