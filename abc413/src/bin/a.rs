use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize, m: usize,
        items: [usize; n],
    };

    println!(
        "{}",
        if items.iter().sum::<usize>() <= m {
            "Yes"
        } else {
            "No"
        }
    );
}
