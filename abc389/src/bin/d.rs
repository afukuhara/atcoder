use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        r: u64
    };

    // in 関数 (クロージャ) の定義
    let in_circle = |a: u64, b: u64| -> bool {
        (2 * a + 1) * (2 * a + 1) + (2 * b + 1) * (2 * b + 1) <= 4 * r * r
    };

    let mut cnt: u64 = 0;
    let mut up = r - 1;
    let mut res = (r - 1) * 4 + 1;

    let mut x = 1;
    while in_circle(x, 1) {
        // 条件を満たすまで up を減少させる
        while !in_circle(x, up) {
            up -= 1;
        }
        cnt += up;
        x += 1;
    }

    res += cnt * 4;
    println!("{}", res);
}
