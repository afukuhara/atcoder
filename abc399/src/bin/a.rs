use proconio::{fastout, input, marker::Chars};

#[fastout]
fn main() {
    input! {
        _n: usize,
        s: Chars,
        t: Chars,
    };

    let ans = s.iter().zip(t.iter()).filter(|(s, t)| s != t).count();
    println!("{}", ans);
}
