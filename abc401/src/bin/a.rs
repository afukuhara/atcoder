#[allow(unused_imports)]
use proconio::{
    fastout, input,
};

#[fastout]
fn main() {
    input! {
        s: usize,
    };

    println!(
        "{}",
        if (200..300).contains(&s) {
            "Success"
        } else {
            "Failure"
        }
    );
}
