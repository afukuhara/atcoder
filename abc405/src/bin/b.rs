use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize, m: usize,
        a: [usize; n],
    };

    for i in 0..=n {
        let mut number_existence = vec![false; m];
        for j in 0..n - i {
            if a[j] <= m {
                number_existence[a[j] - 1] = true;
            }
        }

        if number_existence.iter().any(|&x| !x) {
            println!("{}", i);
            return;
        }
    }
}
