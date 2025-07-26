use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        r: usize, x: usize
    };

    match x {
        1 => println!(
            "{}",
            if (1600..3000).contains(&r) {
                "Yes"
            } else {
                "No"
            }
        ),
        2 => println!(
            "{}",
            if (1200..2400).contains(&r) {
                "Yes"
            } else {
                "No"
            }
        ),
        _ => unreachable!(),
    }
}
