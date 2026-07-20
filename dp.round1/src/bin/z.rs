use proconio::{fastout, input};
use std::collections::VecDeque;

#[derive(Debug, Clone, Copy)]
struct Line {
    m: i128,
    b: i128,
}

impl Line {
    fn eval(&self, x: i128) -> i128 {
        self.m * x + self.b
    }

    fn new(h: i128, dp: i128) -> Line {
        Line {
            m: -2 * h,
            b: dp + h * h,
        }
    }

    fn cross(&self, another: &Line) -> i128 {
        (another.b - self.b) / (self.m - another.m)
    }
}

#[fastout]
fn main() {
    input! {
        n: usize, c: i128,
        h: [i128; n],
    };

    let mut dp = vec![0; n];
    let mut lines = VecDeque::<Line>::new();

    dp[0] = 0;
    lines.push_back(Line::new(h[0], dp[0]));

    for j in 1..n {
        let x = h[j];

        while lines.len() >= 2 && lines[1].eval(x) <= lines[0].eval(x) {
            lines.pop_front();
        }

        let best = lines[0].eval(x);

        dp[j] = x * x + c + best;

        let current_line = Line::new(h[j], dp[j]);
        while lines.len() >= 2 {
            let len = lines.len();
            let (last, second_last) = (lines[len - 1], lines[len - 2]);

            if second_last.cross(&last) <= current_line.cross(&last) {
                break;
            }

            lines.pop_back();
        }

        lines.push_back(current_line);
    }

    println!("{}", dp[n - 1]);
}
