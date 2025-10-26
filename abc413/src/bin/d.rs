use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        t: usize
    };

    for _ in 0..t {
        input! {
            n: usize, mut a: [i128; n]
        }

        // 全部同じ値
        if a.iter().all(|&x| x == a[0]) {
            println!("Yes");
            continue;
        }

        // 先頭とその -1 倍しかなく、それぞれ ceil(N/2) 個と floor(N/2) 個
        if a.iter().all(|&x| x == a[0] || x == -a[0]) {
            let p = a.iter().filter(|&x| *x == a[0]).count();
            let ncnt = a.len() - p;
            if p + ncnt == n && p.min(ncnt) == n / 2 {
                println!("Yes");
                continue;
            }
        }

        a.sort_by_key(|&x| x.abs());

        let r_num = a[1];
        let r_den = a[0];

        let mut ok = true;
        for i in 1..n {
            let lhs = a[i] * r_den;
            let rhs = a[i - 1] * r_num;
            if lhs != rhs {
                ok = false;
                break;
            }
        }

        println!("{}", if ok { "Yes" } else { "No" });
    }
}
