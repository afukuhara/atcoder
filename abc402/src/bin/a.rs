use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        s: String,
    };

    let upper_chars = s.chars().filter(|c| c.is_uppercase()).collect::<String>();

    if upper_chars.len() > 0 {
        println!("{}", upper_chars);
    }
}
