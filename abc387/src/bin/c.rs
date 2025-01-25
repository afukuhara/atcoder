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
        l: Chars, r: Chars,
    };

    // I want to know length of l and r
    println!("{:?} {:?}", l.len(), r.len());

    let mut r_chars = r
        .iter()
        .map(|c| c.to_digit(10).unwrap())
        .collect::<Vec<_>>();

    println!("R number: {:?}", r_chars);

    let r_head = r_chars.remove(0);
    // let r_head_minus_1 = r_head - 1;

    // let from_num = 10_u64.pow((r_chars.len() as u64).try_into().unwrap()) * (r_head as u64);
    // println!("from_num: {:?}", from_num);

    // let mut count: u64 = 1;
    // let mut result = vec![];

    let mut result = 1;
    for i in r_chars {
        println!("i: {:?}", i);
        let count = r_head.min(i);
        result *= count + 1;
    }

    println!("result: {:?}", result);

    // i want to convert result to u64
    // let result_str = result.iter().map(|c| c.to_string()).collect::<String>();
    // println!("result_str: {:?}", result_str);

    // let result_num = result_str.parse::<u64>().unwrap();
    // println!("result_num: {:?}", result_num);
}
