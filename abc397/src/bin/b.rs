use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        s: String
    };

    let mut is_odd = true;
    let mut inserted_num = 0;
    for c in s.chars() {
        if !((is_odd && c == 'i') || (!is_odd && c == 'o')) {
            inserted_num += 1;
            is_odd = !is_odd;
        }

        is_odd = !is_odd;
    }

    if !is_odd {
        inserted_num += 1;
    }

    println!("{}", inserted_num);
}
