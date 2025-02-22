use proconio::{fastout, input, marker::Chars};
use std::cmp::{max, min};

#[fastout]
fn main() {
    input! {
        h: usize, w: usize,
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
    let mut min_x = w;
    let mut min_y = h;

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

    let mut ans = "Yes";

    for line in lines.iter().take(max_x + 1).skip(min_x) {
        for cell in line.iter().take(max_y + 1).skip(min_y) {
            if *cell == '.' {
                ans = "No";
                break;
            }
        }
    }

    println!("{}", ans);
}
