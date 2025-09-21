use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        mut a: [usize; n],
    };

    let mut ans = 0;
    a.sort_unstable_by(|x, y| y.cmp(x));

    for i in (1..=n).rev() {
        if a[i - 1] >= i {
            ans = i;
            break;
        }
    }

    println!("{}", ans);
}
