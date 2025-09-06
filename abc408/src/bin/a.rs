use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize, s: usize,
    };

    input! {
        mut t: [usize; n]
    };

    t.insert(0, 0);
    if t.windows(2).any(|w| w[1] - w[0] > s) {
        println!("No");
    } else {
        println!("Yes");
    }
}
