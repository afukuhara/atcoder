use itertools::Itertools;
use proconio::{fastout, input};
use std::collections::HashMap;

#[fastout]
fn main() {
    input! {
        n: usize,
        p: [usize; n],
    };

    let mut number_of_points = HashMap::new();
    p.iter().for_each(|p| {
        number_of_points.entry(p).or_insert(0);
        number_of_points.entry(p).and_modify(|e| *e += 1);
    });

    let mut current_rank = 1;
    let mut rank_with_count = HashMap::new();
    for (point, rank_count) in number_of_points.iter_mut().sorted_by_key(|(p, _)| *p).rev() {
        rank_with_count.entry(point).or_insert(0);
        rank_with_count
            .entry(point)
            .and_modify(|e| *e += current_rank);
        current_rank += *rank_count;
    }

    for point in p.iter() {
        println!("{}", rank_with_count[&point]);
    }
}
