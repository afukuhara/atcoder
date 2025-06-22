use proconio::{fastout, input};
use regex::Regex;

#[fastout]
fn main() {
    input! {
        t: String,
        u: String,
    };

    let u_len = u.len();
    let t_len = t.len();

    let mut answer = "No";
    for i in 0..(t_len - u_len + 1) {
        let t_slice = &t[i..i + u_len];
        let re = Regex::new(&t_slice.replace("?", ".")).unwrap();

        if re.is_match(&u) {
            answer = "Yes";
            break;
        }
    }

    println!("{}", answer);
}
