use itertools::Itertools;
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        a: [usize; n]
    };

    let set = a.iter().sorted().dedup().collect::<Vec<_>>();

    println!("{}", set.len());
    println!("{}", set.iter().join(" "));
}
