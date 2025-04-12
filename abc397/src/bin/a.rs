use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        x: f32
    };

    println!(
        "{}",
        if x >= 38.0 {
            1
        } else if x < 37.5 {
            3
        } else {
            2
        }
    );
}
