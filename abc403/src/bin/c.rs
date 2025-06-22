use proconio::{fastout, input};
use std::collections::{HashMap, HashSet};

#[fastout]
fn main() {
    input! {
        _n: usize,
        _m: usize,
        q: usize,
    };

    let mut permissions: HashMap<usize, HashSet<isize>> = HashMap::new();
    for _ in 0..q {
        input! {
            query: usize,
        };

        match query {
            1 => {
                input! {
                    x: usize,
                    y: isize,
                };
                permissions.entry(x).or_default().insert(y);
            }
            2 => {
                input! {
                    x: usize,
                };
                permissions.entry(x).or_default().insert(-1);
            }
            3 => {
                input! {
                    x: usize,
                    y: isize,
                };
                let user_permissions = permissions.entry(x).or_default();
                if user_permissions.contains(&y) || user_permissions.contains(&-1) {
                    println!("Yes");
                } else {
                    println!("No");
                }
            }
            _ => unreachable!(),
        }
    }
}
