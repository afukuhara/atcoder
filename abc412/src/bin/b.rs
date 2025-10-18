use proconio::{fastout, input};
use std::collections::HashSet;

#[fastout]
fn main() {
    input! {
        s: String, t: String,
    };

    let chars = s.chars().collect::<Vec<_>>();
    let mut targets: Vec<char> = vec![];

    for i in 1..chars.len() {
        if chars[i].is_ascii_uppercase() {
            targets.push(chars[i - 1]);
        }
    }

    let t_chars = t.chars().collect::<HashSet<_>>();
    let ans = targets.iter().all(|c| t_chars.contains(c));

    println!("{}", if ans { "Yes" } else { "No" });
}
