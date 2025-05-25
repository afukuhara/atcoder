#[allow(unused_imports)]
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize
    };

    let mut ans = 0;
    let mut is_logged_in = false;
    for _ in 1..=n {
        input! {
            s: String,
        };

        match s.as_str() {
            "login" => is_logged_in = true,
            "logout" => is_logged_in = false,
            "private" if is_logged_in => ans += 1,
            _ => {}
        }
    }

    println!("{}", ans);
}
