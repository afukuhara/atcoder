use proconio::{fastout, input};
use std::collections::HashSet;

#[fastout]
fn main() {
    input! {
        _n: usize, r: isize, c: isize,
        s: String,
    };

    let mut current_y: isize = 0;
    let mut current_x: isize = 0;
    let mut map: HashSet<(isize, isize)> = HashSet::new();

    for t in s.chars() {
        map.insert((current_y, current_x));

        match t {
            'N' => current_y -= 1,
            'S' => current_y += 1,
            'E' => current_x += 1,
            'W' => current_x -= 1,
            _ => unreachable!(),
        }

        print!(
            "{}",
            if map.contains(&(current_y - r, current_x - c)) {
                1
            } else {
                0
            }
        );
    }
}
