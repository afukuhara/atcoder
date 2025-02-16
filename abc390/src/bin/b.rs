use num_rational::Rational64;
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        a: [Rational64; n],
    };

    let multiply = a[1] / a[0];
    let mut ans = "Yes";
    for i in 1..n {
        if a[i] != a[i - 1] * multiply {
            ans = "No";
            break;
        }
    }

    println!("{}", ans);
}
