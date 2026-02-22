#[allow(unused_imports)]
use itertools::{iproduct, Itertools};
#[allow(unused_imports)]
use num_traits::pow;
#[allow(unused_imports)]
use proconio::{
    fastout, input,
    marker::{Chars, Usize1},
};
#[allow(unused_imports)]
use std::cmp::{max, min};
#[allow(unused_imports)]
use std::collections::{HashMap, HashSet, VecDeque};
#[allow(unused_imports)]
use std::iter::FromIterator;

#[fastout]
fn main() {
    input! {
        n: usize,
        a: [i128; n]
    };

    let mut dp = vec![vec![-i128::MAX; n + 1]; n + 1];
    let ans = dfs(0, n, &a, &mut dp);
    // println!("{:?} {:?} {:?}", a, dp, ans);
    println!("{}", ans);
}

fn dfs(l: usize, r: usize, a: &[i128], dp: &mut Vec<Vec<i128>>) -> i128 {
    if dp[l][r] != -i128::MAX {
        return dp[l][r];
    }

    if l == r {
        return 0;
    }

    let mut res = -i128::MAX;
    res = max(res, -dfs(l + 1, r, a, dp) + a[l]);
    res = max(res, -dfs(l, r - 1, a, dp) + a[r - 1]);

    dp[l][r] = res;

    // println!("l: {}, r: {}, dp: {:?}", l, r, dp);
    res
}
