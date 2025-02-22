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
        h: usize, _w: usize,
    };

    let mut lines = vec![];
    for _ in 0..h {
        input! {
            s: Chars,
        };
        lines.push(s);
    }

    let mut max_x = 0;
    let mut max_y = 0;
    let mut min_x = 1001;
    let mut min_y = 1001;

    // for i in 0..h {
    //     for j in 0..w {
    //         if lines[i][j] == '#' {
    //             max_x = max(max_x, i);
    //             max_y = max(max_y, j);
    //             min_x = min(min_x, i);
    //             min_y = min(min_y, j);
    //         }
    //     }
    // }

    for (i, line) in lines.iter().enumerate() {
        for (j, c) in line.iter().enumerate() {
            if *c == '#' {
                max_x = max(max_x, i);
                max_y = max(max_y, j);
                min_x = min(min_x, i);
                min_y = min(min_y, j);
            }
        }
    }

    // println!("max: {} {} min: {} {}", max_x, max_y, min_x, min_y);
    let mut ans = "Yes";

    for i in min_x..max_x {
        for j in min_y..max_y {
            if lines[i][j] == '.' {
                ans = "No";
                break;
            }
        }
    }

    println!("{}", ans);
}
