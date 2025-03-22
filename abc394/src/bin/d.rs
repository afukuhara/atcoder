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

    let mut begin_stack = Vec::new();
    let mut end_stack = Vec::new();

    for char in s.chars() {
        if char == '(' || char == '[' || char == '<' {
            begin_stack.push(char);
        } else {
            end_stack.push(char);

            if begin_stack.is_empty() {
                continue;
            }

            let begin = begin_stack.pop().unwrap();
            let end = end_stack.pop().unwrap();

            if !pair_parentheses(begin, end) {
                begin_stack.push(begin);
                end_stack.push(end);
                break;
            }
        }
    }

    println!(
        "{}",
        if begin_stack.is_empty() && end_stack.is_empty() {
            "Yes"
        } else {
            "No"
        }
    );
}

fn pair_parentheses(first: char, second: char) -> bool {
    matches!((first, second), ('(', ')') | ('[', ']') | ('<', '>'))
}
