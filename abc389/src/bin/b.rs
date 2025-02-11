use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
       mut x: u64,
    };

    let mut divide = 2;
    while x != divide {
        x /= divide;
        divide += 1;
    }
    println!("{:?}", divide);
}
