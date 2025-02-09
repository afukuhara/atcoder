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
    };

    input! {
        mut aliens: [usize; n]
    };

    // My code
    // let mut carrie_over = 0;
    // for i in 0..n {
    //     let add = min(i, carrie_over);
    //     aliens[i] += add;
    //     carrie_over -= add;

    //     println!(
    //         "<Middle> i: {:2}, carrie_over: {:2}, aliens: {:?}",
    //         i, carrie_over, aliens
    //     );

    //     let give = min(aliens[i], n - i - 1);
    //     aliens[i] -= give;
    //     carrie_over += give;

    //     println!(
    //         "<After>  i: {:2}, carrie_over: {:2}, aliens: {:?}",
    //         i, carrie_over, aliens
    //     );
    //     println!("--------------------------------");
    // }

    let mut result = vec![0; n];
    let mut diff: Vec<isize> = vec![0; n + 1];
    for i in 0..n {
        if i != 0 {
            result[i] = result[i - 1] + diff[i];
            aliens[i] += result[i] as usize;
        }

        let cnt = min(n - i - 1, aliens[i]);
        aliens[i] -= cnt;
        diff[i + 1] += 1;
        diff[min(n, i + cnt + 1)] -= 1;
    }

    println!("{}", aliens.iter().join(" "));
}
