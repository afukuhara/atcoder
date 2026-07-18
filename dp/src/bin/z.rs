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
        n: usize, c: u64,
        h: [u64; n],
    };

    let mut dp = vec![u64::MAX; n];
    dp[0] = 0;

    for i in 0..n - 1 {
        for j in i + 1..n {
            let cost = (h[j] - h[i]).pow(2) + c;
            dp[j] = min(dp[j], dp[i] + cost);

            // println!(
            //     "i: {}, j: {}, h[i]: {}, h[j]: {}, cost: {}, dp[j]: {}, dp[i]: {}",
            //     i, j, h[i], h[j], cost, dp[j], dp[i]
            // );
        }

        // println!("---------------");
        // println!("dp: {:?}", dp);
        // println!("--------------------------------");
    }

    // println!("{:?}", dp);
    println!("{}", dp[n - 1]);
}
