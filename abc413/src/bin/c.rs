use proconio::{fastout, input};
use std::collections::VecDeque;

#[fastout]
fn main() {
    input! {
        q: usize
    };

    let mut stack = VecDeque::new();
    for _ in 0..q {
        input! { t: usize };

        match t {
            1 => {
                input! { count: usize, x: usize  };
                stack.push_back((count, x));
            }
            2 => {
                input! { mut k: usize  };

                let mut answer = 0;
                while k > 0 {
                    let (count, x) = stack.pop_front().unwrap_or((0, 0));
                    if count > k {
                        stack.push_front((count - k, x));
                        answer += x * k;
                        k = 0;
                    } else {
                        answer += x * count;
                        k -= count;
                    }
                }

                println!("{}", answer);
            }
            _ => unreachable!(),
        }
    }
}
