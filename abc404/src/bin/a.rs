use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        s: String,
    };

    let alphabet = "abcdefghijklmnopqrstuvwxyz";

    for c in alphabet.chars() {
        if !s.contains(c) {
            println!("{}", c);
            return;
        }
    }
}
