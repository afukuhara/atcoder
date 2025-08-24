use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        a: usize, b: usize,
    };

    println!("{}", (a as f64 / b as f64).round() as usize);
}
