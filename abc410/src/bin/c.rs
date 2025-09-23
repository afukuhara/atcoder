use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize, q: usize,
    };

    let mut array = (1..=n).collect::<Vec<usize>>();
    let mut offset = 0;

    for _ in 0..q {
        input! { t: usize };

        match t {
            1 => {
                input! { p: usize, x: usize };
                array[(p - 1 + offset) % n] = x;
            }
            2 => {
                input! { p: usize };
                println!("{}", array[(p - 1 + offset) % n]);
            }
            3 => {
                input! { k: usize };
                offset = (offset + k) % n;
            }
            _ => unreachable!(),
        }
    }
}
