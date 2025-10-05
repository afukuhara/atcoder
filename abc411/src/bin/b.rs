use itertools::Itertools;
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        d: [usize; n-1],
    };

    let mut i = 0;
    while n - i > 1 {
        let mut distances = vec![];
        let mut current = 0;
        for d in d.iter().skip(i) {
            current += d;
            distances.push(current);
        }

        i += 1;
        println!("{}", distances.iter().join(" "));
    }
}
