use proconio::{fastout, input};
use std::collections::HashMap;

#[fastout]
fn main() {
    input! {
        n: usize, q: usize,
    };

    let mut pigeons = HashMap::new();
    let mut nests = HashMap::new();

    for i in 1..n + 1 {
        pigeons.entry(i).or_insert(i);
    }

    let mut multi_counts = 0;

    for _ in 0..q {
        input! {
            q: usize,
        }

        match q {
            1 => {
                input! {
                    p: usize, h: usize,
                }

                let nest = pigeons.get_mut(&p).unwrap();

                let prev_nest_pigeon_count = *nests.entry(*nest).or_insert(1);
                if prev_nest_pigeon_count == 2 {
                    multi_counts -= 1;
                }

                let next_nest_pigeon_count = *nests.entry(h).or_insert(1);
                if next_nest_pigeon_count == 1 {
                    multi_counts += 1;
                }

                *nests.entry(h).or_insert(0) += 1;
                *nests.entry(*nest).or_insert(0) -= 1;

                *pigeons.entry(p).or_insert(0) = h;
            }
            2 => {
                println!("{}", multi_counts);
            }
            _ => {
                unreachable!();
            }
        }
    }
}
