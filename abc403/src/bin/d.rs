use proconio::{fastout, input};
use std::cmp::max;

#[fastout]
fn main() {
    input! {
        n: usize, d: usize,
    };

    const INITIAL_MAX_NUM: usize = 1000005;

    let mut nums = vec![0; INITIAL_MAX_NUM];
    let mut max_num = 0;
    for _ in 0..n {
        input! {
            a: usize,
        };

        nums[a] += 1;
        max_num = max(max_num, a);
    }

    let mut ans = 0;

    if d == 0 {
        let kinds = nums.iter().filter(|&x| *x > 0).count();
        ans = n - kinds;
    } else {
        for si in 0..d {
            let mut numbers = vec![];
            for i in (si..=max_num).step_by(d) {
                numbers.push(nums[i]);
            }

            ans += solve(&numbers);
        }
    }

    println!("{}", ans);
}

fn solve(nums: &[usize]) -> usize {
    let n = nums.len();
    let mut dp = vec![vec![0; 2]; n + 1];

    for i in 0..n {
        dp[i + 1][1] = max(dp[i][0], dp[i][1]);
        dp[i + 1][0] = dp[i][1] + nums[i];
    }

    let mx = max(dp[n][0], dp[n][1]);
    nums.iter().sum::<usize>() - mx
}
