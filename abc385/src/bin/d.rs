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

static X_MAX: i64 = 1_000_000_000;
static Y_MAX: i64 = 1_000_000_000;
static X_MIN: i64 = -1_000_000_000;
static Y_MIN: i64 = -1_000_000_000;

#[fastout]
fn main() {
    input!{
        n: u32, m: u32, sx: i64, sy: i64
    };

    let mut houses: Vec<(i64, i64)> = Vec::new();
    for _ in 0..n {
        input!{
            x: i64, y: i64
        };
        houses.push((x, y));
    }

    let mut instructions: Vec<(char, i64)> = Vec::new();
    for _ in 0..m {
        input!{
            c: char, d: i64
        };
        instructions.push((c, d));
    }


    let mut santa_pos = (sx, sy);
    let mut result = 0;
    
    for (direction, distance) in instructions {
        let prev_pos = santa_pos;
        match direction {
            'U' => {
                santa_pos.1 = santa_pos.1 + distance;
                result += remove_houses_vertical(&mut houses, prev_pos.0, prev_pos.1, santa_pos.1);
            }
            'D' => {
                santa_pos.1 = santa_pos.1 - distance;
                result += remove_houses_vertical(&mut houses, prev_pos.0, santa_pos.1, prev_pos.1);
            }
            'L' => {
                santa_pos.0 = santa_pos.0 - distance;
                result += remove_houses_horizontal(&mut houses, prev_pos.1, santa_pos.0, prev_pos.0);
            }
            'R' => {
                santa_pos.0 = santa_pos.0 + distance;
                result += remove_houses_horizontal(&mut houses, prev_pos.1, prev_pos.0, santa_pos.0);
            }
            _ => unreachable!(),
        }

        // println!("Current position: {} {}", santa_pos.0, santa_pos.1);
    }

    println!("{} {} {}", santa_pos.0, santa_pos.1, result);
}

fn remove_houses_vertical(houses: &mut Vec<(i64, i64)>, x_const: i64, lower_y: i64, upper_y: i64) -> i64 {
    if x_const >  X_MAX || x_const < X_MIN || lower_y > Y_MAX || upper_y < Y_MIN {
        return 0;
    }

    let mut result = 0;
    houses.retain(|&(x, y)| {
        if x == x_const && (lower_y <= y && y <= upper_y) {
            result += 1;
            false
        } else {
            true
        }
    });
    result
}

fn remove_houses_horizontal(houses: &mut Vec<(i64, i64)>, y_const: i64, lower_x: i64, upper_x: i64) -> i64 {
    if y_const > Y_MAX || y_const < Y_MIN || lower_x > X_MAX || upper_x < X_MIN {
        return 0;
    }

    let mut result = 0;
    houses.retain(|&(x, y)| {
        if y == y_const && (lower_x <= x && x <= upper_x) {
            result += 1;
            false
        } else {
            true
        }
    });
    result
}