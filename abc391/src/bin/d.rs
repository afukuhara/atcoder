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
        n: usize, w: usize,
    };

    let mut grid = vec![vec![0; n + 1]; w];
    let mut existing_blocks = HashSet::new();
    let mut offsets = vec![0; w];

    for i in 0..n {
        input! {
            x: usize, y: usize,
        }

        grid[x - 1][y - 1] = i + 1;
        existing_blocks.insert(i + 1);
    }

    // _show_grid(&grid, 0, &offsets);

    input! {
        q: usize,
    }

    let mut current_time = 0;

    for _ in 0..q {
        input! {
            time: usize, a: usize,
        }

        while current_time < time {
            // move blocks
            let mut is_all = true;
            let mut will_be_removed = HashSet::new();
            for x in 0..w {
                let current_offset = offsets[x];

                let value = grid[x][current_offset];
                if value == 0 {
                    offsets[x] += 1;
                    is_all = false;
                } else {
                    will_be_removed.insert(value);
                }
            }

            if is_all {
                will_be_removed.iter().for_each(|value| {
                    existing_blocks.remove(value);
                });
                offsets.iter_mut().for_each(|offset| *offset += 1);
            }
            current_time += 1;
        }

        // _show_grid(&grid, current_time, &offsets);

        println!(
            "{}",
            if existing_blocks.contains(&a) {
                "Yes"
            } else {
                "No"
            }
        );
    }

    // _show_grid(&grid, current_time, &offsets);
}

fn _show_grid(grid: &[Vec<usize>], current_time: usize, offsets: &[usize]) {
    println!("TIME: {}", current_time);
    for (i, row) in grid.iter().enumerate() {
        let offset = offsets[i];
        println!("{:?}", row[offset..].to_vec());
    }
}
