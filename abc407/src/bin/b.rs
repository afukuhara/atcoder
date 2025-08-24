use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        x: isize, y: isize,
    };

    let mut two_dice = vec![];
    for i in 1..=6 {
        for j in 1..=6 {
            two_dice.push((i, j));
        }
    }

    let count = two_dice
        .iter()
        .filter(|(a, b)| a + b >= x || (a - b).abs() >= y)
        .count();

    println!("{}", count as f64 / 36.0);
}
