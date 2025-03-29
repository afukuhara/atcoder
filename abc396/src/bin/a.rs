use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        nums: [usize; n],
    };

    if nums.windows(3).any(|w| w[0] == w[1] && w[1] == w[2]) {
        println!("Yes");
    } else {
        println!("No");
    }
}
