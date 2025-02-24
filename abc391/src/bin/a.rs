use proconio::{fastout, input};
use std::collections::HashMap;

#[fastout]
fn main() {
    input! {
        d: String,
    };

    let hash = HashMap::from([
        ("N", "S"),
        ("S", "N"),
        ("E", "W"),
        ("W", "E"),
        ("NE", "SW"),
        ("SW", "NE"),
        ("SE", "NW"),
        ("NW", "SE"),
    ]);

    println!("{}", hash[d.as_str()]);
}
