use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        s: String,
    };

    let mut chars = s.chars().collect::<Vec<_>>();
    let mut charge_w = 0;

    for i in 0..chars.len() {
        if chars[i] == 'W' {
            charge_w += 1;
        } else if chars[i] == 'A' && charge_w > 0 {
            chars[i - charge_w] = 'A';
            for j in 0..charge_w {
                chars[i - j] = 'C';
            }
            charge_w = 0;
        } else {
            charge_w = 0;
        }
    }

    println!("{}", chars.iter().collect::<String>());
}
