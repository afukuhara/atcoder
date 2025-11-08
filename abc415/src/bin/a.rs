use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        a: [usize; n],
        x: usize,
    };

    println!(
        "{}",
        if a.iter().any(|&a| a == x) {
            "Yes"
        } else {
            "No"
        }
    );
}
