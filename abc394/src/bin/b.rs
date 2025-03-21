use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
    };

    let mut strings = Vec::new();
    for _ in 0..n {
        input! {
            s: String,
        };
        strings.push(s);
    }

    strings.sort_by_key(|s| s.len());
    println!("{}", strings.concat());
}
