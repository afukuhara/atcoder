use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize, l: usize,
        d: [usize; n - 1],
    };

    if l % 3 != 0 {
        println!("0");
        return;
    }

    let mut points = vec![0_u128; l];
    points[0] = 1;

    let mut position = 0;
    for i in d {
        position = (position + i) % l;
        points[position] += 1;
    }

    let step = l / 3;
    let mut count = 0;
    for i in 0..step {
        count += points[i] * points[i + step] * points[i + 2 * step];
    }

    println!("{}", count);
}
