use proconio::{fastout, input};
use std::cmp::{max, min};
use std::collections::{HashMap, HashSet};

#[fastout]
fn main() {
    input! {
        t: usize,
    };

    for _ in 0..t {
        input! {
            n: usize,
        }

        let mut people = vec![0; n * 2];
        for i in 0..n * 2 {
            input! {
                a: usize
            }
            people[i] = a;
        }

        let mut num_position = HashMap::new();
        for (i, p) in people.iter().enumerate() {
            num_position.entry(p).or_insert(vec![]).push(i);
        }

        let mut answer = HashSet::new();
        for i in 0..n * 2 - 1 {
            let a = people[i];
            let b = people[i + 1];

            let pos_a = num_position.get(&a).unwrap();
            let pos_b = num_position.get(&b).unwrap();
            if pos_a[0] + 1 == pos_a[1] || pos_b[0] + 1 == pos_b[1] {
                continue;
            }

            let mut pairs = [pos_a[0], pos_a[1], pos_b[0], pos_b[1]];
            pairs.sort();
            if pairs[0] + 1 == pairs[1] && pairs[2] + 1 == pairs[3] {
                answer.insert((min(pairs[0], pairs[2]), max(pairs[0], pairs[2])));
            }
        }

        println!("{}", answer.len());
    }
}
