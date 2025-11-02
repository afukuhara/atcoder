use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        s: [(char, u128); n],
    };

    let mut answer = String::new();
    let mut total = 0;
    for (c, x) in s {
        total += x;

        if total > 100 {
            println!("Too Long");
            return;
        }

        answer.push_str(&c.to_string().repeat(x as usize));
    }

    println!("{}", answer);
}
