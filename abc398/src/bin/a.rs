use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize
    };

    let (half, equal) = if n % 2 == 0 {
        (n / 2 - 1, "==")
    } else {
        (n / 2, "=")
    };

    println!("{}{}{}", "-".repeat(half), equal, "-".repeat(half));
}
