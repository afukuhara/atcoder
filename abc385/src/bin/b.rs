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

use std::io;

#[fastout]
fn solve() -> io::Result<()> {
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;

    let nums: Vec<usize> = input
        .split_ascii_whitespace()
        .map(|s| s.parse::<usize>().unwrap())
        .collect();
    let [h, _w, mut x, mut y] = nums[..] else {
        panic!("Expected 2 numbers")
    };

    let mut rows: Vec<Vec<char>> = Vec::new();
    for _i in 0..h {
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        let row: Vec<char> = input.trim_end().chars().collect();
        rows.push(row);
    }

    // println!("{:?}", rows);

    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    let instructions: Vec<char> = input.trim_end().chars().collect();

    // println!("{:?}", instructions);

    let mut result = 0;
    let mut visited = HashSet::new();
    visited.insert((x, y));

    for inst in instructions {
        // println!("{} {} {}", x, y, inst);

        let prev = (x, y);
        match inst {
            'U' => x -= 1,
            'D' => x += 1,
            'L' => y -= 1,
            'R' => y += 1,
            _ => unreachable!(),
        }

        match rows[x - 1][y - 1] {
            '.' => {}
            '#' => {
                x = prev.0;
                y = prev.1;
            }
            '@' => {
                if !visited.contains(&(x, y)) {
                    visited.insert((x, y));
                    result += 1;
                }
            }
            _ => unreachable!(),
        }

        // println!("!! {} {} {}", result.0, result.1, result.2);
    }

    println!("{} {} {}", x, y, result);

    Ok(())
}

pub fn main() {
    if let Err(e) = solve() {
        eprintln!("Error: {}", e);
    }
}
