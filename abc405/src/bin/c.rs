use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        nums: [usize; n],
    };

    let mut ans = 0usize;
    let mut suffix_sum = nums.iter().sum::<usize>();

    for &x in &nums {
        suffix_sum -= x;
        ans += x * suffix_sum;
    }

    println!("{}", ans);
}
