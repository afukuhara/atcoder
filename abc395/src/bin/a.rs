use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        a: [usize; n],
    };

    let answer = if a.windows(2).any(|a| a[0] >= a[1]) {
        "No"
    } else {
        "Yes"
    };

    println!("{}", answer);
}
