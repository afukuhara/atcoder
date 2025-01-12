use proconio::{fastout, input};
use std::collections::BTreeSet;

#[fastout]
fn main() {
    input! {
        n: u32, m: u32,
    };

    let mut cells: BTreeSet<(u32, u32, char)> = BTreeSet::new();

    for _i in 0..m {
        input! {
            x: u32, y: u32, c: char,
        };

        cells.insert((x, y, c));
    }

    let mut min_y = n + 1;
    for (_, y, c) in cells {
        if c == 'W' {
            min_y = min_y.min(y);
        } else if min_y <= y {
            println!("No");
            return;
        }
    }

    println!("Yes");
}
