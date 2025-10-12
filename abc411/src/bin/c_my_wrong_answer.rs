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
        n: usize, q: usize,
    };

    // let mut array = vec![0; n];

    let mut btreemap = BTreeMap::new();

    for _ in 0..q {
        input! { a: Usize1 };

        let left = if a == 0 { 0 } else { a - 1 };
        let right = if a == n - 1 { n - 1 } else { a + 1 };

        // println!("--------------------------------");
        // println!("left: {}, a: {}, right: {}", left, a, right);

        if let Some((&l, &r)) = btreemap.range(left..=right).last() {
            // println!("HIT, l: {}, r: {}, a: {}", l, r, a);
            if l == r && l == a {
                btreemap.remove(&l);
            } else if r == left {
                btreemap.insert(l, a);
            } else if l == right {
                btreemap.remove(&l);
                btreemap.insert(a, r);
            } else if l == a {
                btreemap.remove(&l);
                btreemap.insert(right, r);
            } else if r == a {
                btreemap.insert(l, left);
            } else {
                btreemap.remove(&l);
                btreemap.insert(l, a);
                btreemap.insert(right, r);
            }
        } else {
            btreemap.insert(a, a);
        }

        // println!("btreemap: {:?}", btreemap);
        let mut ranges: Vec<(usize, usize)> = vec![];
        let mut prev_right = usize::MAX - 1;
        for (k, v) in btreemap.iter() {
            if *k == prev_right + 1 {
                ranges.last_mut().unwrap().1 = *v;
            } else {
                ranges.push((*k, *v));
            }

            prev_right = *v;
        }

        btreemap.clear();
        for (l, r) in ranges {
            btreemap.insert(l, r);
        }

        println!("{}", btreemap.len());
    }
}
