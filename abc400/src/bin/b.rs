use proconio::{fastout, input};
use num_traits::pow;

#[fastout]
fn main() {
    input! {
        n: i64, m: u32,
    };

    let mut ans: i64 = 0;
    for i in 0..=m {
        ans += n.pow(i);
        if ans > 1_000_000_000 {
            ans = -1;
            break;
        }
    }

    if ans == -1 {
        println!("{}", "inf");
    } else {
        println!("{}", ans);
    }
}
