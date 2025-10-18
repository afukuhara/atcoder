use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize
    };

    let mut count = 0;
    for _ in 0..n {
        input! { a: usize, b: usize };
        if a < b {
            count += 1;
        }
    }

    println!("{}", count);
}
