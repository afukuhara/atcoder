use proconio::{fastout, input};

#[fastout]
fn main() {
    const MOD: i128 = 1_000_000_000;

    input! {
        n: usize, k: usize,
    };

    let mut a = vec![1; n + 1];
    let mut s = k as i128;

    for i in k..n + 1 {
        a[i] = s;
        s -= a[i - k];
        s += a[i];
        s = ((s % MOD) + MOD) % MOD; // “正の mod” に正規化
    }
    println!("{}", a[n]);
}
