use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize, k: usize,
        nums: [u128; n],
    };

    let mut ans: u128 = 1;
    let max_digit = 10_u128.pow(k as u32);
    for num in nums {
        let tmp_num = ans * num;
        ans = if tmp_num >= max_digit { 1 } else { tmp_num };
    }

    println!("{}", ans);
}
