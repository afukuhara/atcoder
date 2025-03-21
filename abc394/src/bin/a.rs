use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        s: String,
    };

    let answer = s.chars().filter(|&c| c == '2').collect::<String>();
    println!("{}", answer);
}
