use proconio::{fastout, input};
use std::collections::{BTreeSet, HashMap};

#[fastout]
fn main() {
    input! {
        n: u32, m: u32, sx: i64, sy: i64
    };

    let mut houses_x: HashMap<i64, BTreeSet<i64>> = HashMap::new();
    let mut houses_y: HashMap<i64, BTreeSet<i64>> = HashMap::new();
    for _ in 0..n {
        input! {
            x: i64, y: i64
        };
        houses_x.entry(x).or_default().insert(y);
        houses_y.entry(y).or_default().insert(x);
    }

    let mut instructions: Vec<(char, i64)> = Vec::new();
    for _ in 0..m {
        input! {
            c: char, d: i64
        };
        instructions.push((c, d));
    }

    let mut santa_pos = (sx, sy);
    let mut result = 0;

    for (direction, distance) in instructions {
        let prev_pos = santa_pos;
        match direction {
            'U' => {
                santa_pos.1 += distance;
                result += remove_houses(
                    &mut houses_x,
                    &mut houses_y,
                    prev_pos.0,
                    prev_pos.1,
                    santa_pos.1,
                );
            }
            'D' => {
                santa_pos.1 -= distance;
                result += remove_houses(
                    &mut houses_x,
                    &mut houses_y,
                    prev_pos.0,
                    santa_pos.1,
                    prev_pos.1,
                );
            }
            'L' => {
                santa_pos.0 -= distance;
                result += remove_houses(
                    &mut houses_y,
                    &mut houses_x,
                    prev_pos.1,
                    santa_pos.0,
                    prev_pos.0,
                );
            }
            'R' => {
                santa_pos.0 += distance;
                result += remove_houses(
                    &mut houses_y,
                    &mut houses_x,
                    prev_pos.1,
                    prev_pos.0,
                    santa_pos.0,
                );
            }
            _ => unreachable!(),
        }
    }

    println!("{} {} {}", santa_pos.0, santa_pos.1, result);
}

fn remove_houses(
    outer_map: &mut HashMap<i64, BTreeSet<i64>>,
    inner_map: &mut HashMap<i64, BTreeSet<i64>>,
    coord: i64,
    min_val: i64,
    max_val: i64,
) -> i64 {
    let mut count = 0;
    if let Some(houses) = outer_map.get(&coord) {
        let to_remove = houses.range(min_val..=max_val).copied().collect::<Vec<_>>();

        for pos in to_remove {
            count += 1;
            outer_map.get_mut(&coord).unwrap().remove(&pos);
            inner_map.get_mut(&pos).unwrap().remove(&coord);
        }
    }
    count
}

fn dfs(
    idx: usize,
    group_count: usize,
    stone_bags: &Vec<usize>,
    bag_groups: &mut Vec<usize>,
    xor_result: &mut usize,
    answers: &mut HashSet<usize>,
) {
    // Function implementation...
}
