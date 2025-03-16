use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        s1: String,
        s2: String,
    };

    let s1 = s1.as_str();
    let s2 = s2.as_str();
    match (s1, s2) {
        ("sick", "sick") => println!("1"),
        ("sick", "fine") => println!("2"),
        ("fine", "sick") => println!("3"),
        ("fine", "fine") => println!("4"),
        _ => unreachable!(),
    }
}
