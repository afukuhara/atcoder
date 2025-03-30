use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
    };

    let mut stack = vec![0; 100];

    for _ in 0..n {
        input! {
            op: usize,
        }

        match op {
            1 => {
                input! {
                    target: usize,
                }
                stack.push(target);
            }
            2 => {
                let last = stack.pop().unwrap();
                println!("{}", last);
            }
            _ => unreachable!(),
        }
    }
}
