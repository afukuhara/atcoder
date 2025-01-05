use proconio::{fastout, input, marker::Chars};

#[fastout]
fn main() {
    input! {
        s: Chars,
    };

    let mut answer = 0;
    let mut iter = s.iter();

    while let Some(c) = iter.next() {
        if *c == '0' {
            match iter.next() {
                Some('0') => answer += 1,
                Some(_) => answer += 2,
                None => answer += 1,
            }
        } else {
            answer += 1;
        }
    }

    println!("{}", answer);
}
