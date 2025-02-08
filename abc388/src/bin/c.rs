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
use std::collections::{BTreeMap, HashMap, HashSet, VecDeque};
#[allow(unused_imports)]
use std::iter::FromIterator;

#[fastout]
fn main() {
    input! {
        n: usize,
    };

    input! {
        mochi: [usize; n]
    };

    // my original code
    // let mut sorted_mochi: BTreeMap<&usize, usize> = BTreeMap::new();
    // let mut result = 0;

    // mochi.iter().for_each(|m| {
    //     let max = m / 2;
    //     let valid_mochi_count = sorted_mochi.range(0..=max).map(|(_, v)| v).sum::<usize>();
    //     result += valid_mochi_count;
    //     *sorted_mochi.entry(m).or_default() += 1;
    // });

    let mut result = 0;
    let mut j = 0; // a / 2 より大きい最初の要素（なければ最後の次）のインデックス

    for &a in &mochi {
        // Advance j until A[j] is greater than a/2 (i.e. while A[j]*2 <= a).
        while j < n && mochi[j] * 2 <= a {
            j += 1;
        }
        result += j;
    }

    println!("{}", result);
}
