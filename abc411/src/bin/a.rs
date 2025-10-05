use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        p: String, l: usize,
    };

    println!("{}", if p.len() >= l { "Yes" } else { "No" });
}
