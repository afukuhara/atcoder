use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize, l: usize, r: usize,
    };

    let mut listner = 0;
    for _ in 0..n {
        input! {
            x: usize, y: usize,
        }

        if x <= l && r <= y {
            listner += 1;
        }
    }

    println!("{}", listner);
}
