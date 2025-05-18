use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: u128,
    };

    let mut a: i32 = 60;
    let mut res: u128 = 0;
    let mut b: u128 = 1;

    while a > 0 {
        while a > 0 && (2u128.pow(a as u32) * b * b > n) {
            a -= 1;
        }
        res += a as u128;
        b += 2;
    }

    println!("{}", res);
}
