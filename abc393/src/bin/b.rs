use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        s: String,
    };

    let chars = s.chars().collect::<Vec<_>>();

    let mut count = 0;
    for step in 1..(chars.len() - 1) {
        for offset in 0..step {
            count += chars
                .iter()
                .skip(offset)
                .step_by(step)
                .collect::<Vec<_>>()
                .windows(3)
                .filter(|w| *w == [&'A', &'B', &'C'])
                .count();
        }
    }
    println!("{}", count);
}
