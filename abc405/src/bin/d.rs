use proconio::{fastout, input};
use std::collections::VecDeque;

#[fastout]
fn main() {
    input! {
        h: usize, w: usize,
    };

    let mut matrix = vec![];
    let mut queue = VecDeque::new();
    for i in 0..h {
        input! { s: String };

        let row = s.chars().collect::<Vec<char>>();
        for (j, &c) in row.iter().enumerate() {
            if c == 'E' {
                queue.push_back((i as isize, j as isize));
            }
        }
        matrix.push(row);
    }

    let dx: Vec<isize> = vec![1, 0, -1, 0];
    let dy: Vec<isize> = vec![0, 1, 0, -1];
    let direction_marks = ['^', '<', 'v', '>'];
    let (h_i, w_i) = (h as isize, w as isize);

    while let Some((x, y)) = queue.pop_front() {
        for i in 0..4 {
            let nx = x + dx[i];
            let ny = y + dy[i];

            if nx < 0 || nx >= h_i || ny < 0 || ny >= w_i {
                continue;
            }

            let (ux, uy) = (nx as usize, ny as usize);

            if matrix[ux][uy] != '.' {
                continue;
            }
            matrix[ux][uy] = direction_marks[i];
            queue.push_back((nx, ny));
        }
    }

    for row in matrix {
        println!("{}", row.iter().collect::<String>());
    }
}
