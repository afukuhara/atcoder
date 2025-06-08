use proconio::{fastout, input};
use std::collections::VecDeque;

#[fastout]
fn main() {
    input! {
        n: usize,
    };

    let mut dq = VecDeque::new();

    for _ in 0..n {
        input! {
            q: usize,
        }

        if q == 1 {
            input! {
                x: usize,
            }

            dq.push_back(x);
        } else if q == 2 {
            println!("{}", dq.pop_front().unwrap());
        }
    }
}
