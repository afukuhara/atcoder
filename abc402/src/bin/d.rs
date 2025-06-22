use proconio::{fastout, input};
use std::collections::HashMap;

#[fastout]
fn main() {
    input! {
        n: usize, m: usize,
    };

    let mut line_mods = HashMap::new();

    for _ in 0..m {
        input! {
            a: usize, b: usize,
        };

        let mod_value = (a + b) % n;
        *line_mods.entry(mod_value).or_insert(0) += 1;
    }

    let line_mods_sum = line_mods
        .into_values()
        .map(|v| v * (v - 1) / 2)
        .sum::<usize>();

    let total_line_combination = m * (m - 1) / 2;
    println!("{}", total_line_combination - line_mods_sum);
}
