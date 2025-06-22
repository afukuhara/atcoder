use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
    };

    let mut answer = 0;
    for i in 1..=n {
        input! { a: usize };

        if i % 2 != 0 {
            answer += a;
        }
    }

    println!("{}", answer);
}
