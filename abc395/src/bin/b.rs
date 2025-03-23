use itertools::Itertools;
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
    };

    let mut chars = vec![vec!['#'; n]; n];

    let mut width: i32 = n as i32 - 2;
    let mut offset: i32 = 1;

    while width > 0 {
        draw_square(&mut chars, width as usize, offset as usize);
        width -= 4;
        offset += 2;
    }

    for line in chars.iter_mut() {
        println!("{}", line.iter().join(""));
    }
}

fn draw_square(chars: &mut [Vec<char>], size: usize, offset: usize) {
    for y in 0..size {
        chars[y + offset][offset] = '.';
        chars[y + offset][offset + size - 1] = '.';
    }

    for x in 0..size {
        chars[offset][x + offset] = '.';
        chars[offset + size - 1][x + offset] = '.';
    }
}
