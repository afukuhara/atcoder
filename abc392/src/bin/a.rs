use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        a1: usize, a2: usize, a3: usize,
    };

    if a1 * a2 == a3 {
        println!("Yes");
    } else if a1 * a3 == a2 {
        println!("Yes");
    } else if a2 * a3 == a1 {
        println!("Yes");
    } else {
        println!("No");
    }
}
