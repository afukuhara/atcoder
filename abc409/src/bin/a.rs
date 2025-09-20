use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        _n: usize,
        t: String,
        a: String,
    };

    let ans = t.chars().zip(a.chars()).any(|(t, a)| (t, a) == ('o', 'o'));
    println!("{}", if ans { "Yes" } else { "No" });
}
