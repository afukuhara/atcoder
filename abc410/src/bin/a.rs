use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        a: [usize; n],
        k: usize,
    };

    let count = a.iter().filter(|&x| *x >= k).count();
    println!("{}", count);
}
