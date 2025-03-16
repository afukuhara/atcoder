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
        s: String,
    };

    let chars = s.chars().collect::<Vec<_>>();

    let mut count = 0;
    for step in 1..(chars.len() - 1) {
        for offset in 0..step {
            let subseq = chars.iter().skip(offset).step_by(step);

            if subseq.len() < 3 {
                continue;
            }

            for k in 0..subseq.len() {
                if subseq.clone().skip(k).take(3).eq(&['A', 'B', 'C']) {
                    count += 1;
                }
            }
        }
    }
    println!("{}", count);
}
