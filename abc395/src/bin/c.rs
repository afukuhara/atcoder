use proconio::{fastout, input};
use std::collections::HashMap;

#[fastout]
fn main() {
    input! {
        n: usize,
        nums: [isize; n],
    };

    let mut hash: HashMap<isize, Vec<isize>> = HashMap::new();

    for (i, &num) in nums.iter().enumerate() {
        hash.entry(num).or_default().push(i as isize);
    }

    let ans: isize = hash
        .iter()
        .filter(|(_, v)| v.len() > 1)
        .flat_map(|(_, v)| v.windows(2))
        .map(|w| w[1] - w[0] + 1)
        .min()
        .unwrap_or(-1);

    println!("{}", ans);
}
