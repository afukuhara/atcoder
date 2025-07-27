use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        a: usize, b: usize,
        c: usize, d: usize,
    };

    if c < a || (c == a && d < b) {
        println!("{}", "Yes");
    } else {
        println!("{}", "No");
    }
}
