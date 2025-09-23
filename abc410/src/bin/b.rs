use itertools::Itertools;
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize, q: usize,
        x: [usize; q],
    };

    let mut boxes = vec![0; n];
    let mut balls = vec![0; q];

    for (i, x) in x.iter().enumerate() {
        match x {
            0 => {
                let minimum = boxes.iter().min().unwrap();
                for (j, b) in boxes.iter().enumerate() {
                    if b == minimum {
                        boxes[j] += 1;
                        balls[i] = j;
                        break;
                    }
                }
            }
            _ => {
                boxes[x - 1] += 1;
                balls[i] = x - 1;
            }
        }
    }

    println!("{}", balls.iter().map(|x| x + 1).join(" "));
}
