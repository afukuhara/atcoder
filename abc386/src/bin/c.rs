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
        _k: usize,
        s: Chars,
        t: Chars,
    };

    let len_s = s.len();
    let len_t = t.len();
    if (len_s as isize - len_t as isize).abs() > 1 {
        // 1 回の操作では無理
        println!("No");
        return;
    }

    let mut answer = "Yes";
    if len_s == len_t {
        let mut diff_count = 0;
        let mut s_iter = s.iter();
        let mut t_iter = t.iter();

        while let Some(s_row) = s_iter.next() {
            let t_row = t_iter.next().unwrap();

            if t_row != s_row {
                if diff_count == 0 {
                    diff_count = 1;
                } else {
                    answer = "No";
                    break;
                }
            }
        }
    } else {
        let (mut small_iter, mut large_iter) = if s.len() > t.len() {
            (t.iter(), s.iter())
        } else {
            (s.iter(), t.iter())
        };

        let mut diff_count = 0;
        while let Some(large_row) = large_iter.next() {
            let small_row = small_iter.next();

            if small_row.is_none() {
                if diff_count == 0 {
                    answer = "Yes";
                } else {
                    answer = "No";
                }
                break;
            }

            // println!("{} {}", large_row, small_row);
            if large_row != small_row.unwrap() {
                if diff_count == 0 {
                    diff_count = 1;

                    let large_row_next = large_iter.next();
                    // println!("{} {}", large_row_next, small_row);

                    if large_row_next.is_none() {
                        // answer = "No";
                        break;
                    }

                    if large_row_next.unwrap() != small_row.unwrap() {
                        answer = "No";
                        break;
                    }
                } else {
                    answer = "No";
                    break;
                }
            }
        }
    }

    println!("{}", answer);
}
