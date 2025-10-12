use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize, q: usize,
    };

    let mut cells = vec![0; n + 1];
    let mut count = 0;

    for _ in 0..q {
        input! { a: usize };

        count += (1 - cells[a - 1]) - cells[a - 1];
        count += (1 - cells[a]) - cells[a];

        cells[a - 1] = 1 - cells[a - 1];
        cells[a] = 1 - cells[a];

        println!("{}", count / 2);
    }
}
