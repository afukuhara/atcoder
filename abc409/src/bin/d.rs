use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        t: usize,
    };

    for _ in 0..t {
        input! {
            n: usize,
            s: String,
        };

        solve(&s.chars().collect::<Vec<_>>(), n);
    }
}

fn solve(st: &[char], n: usize) {
    let mut l = usize::MAX;
    for i in 0..n - 1 {
        if st[i] > st[i + 1] {
            l = i;
            break;
        }
    }

    if l == usize::MAX {
        println!("{}", st.iter().collect::<String>());
        return;
    }

    let mut r = n;
    for i in l + 1..r {
        if st[l] < st[i] {
            r = i;
            break;
        }
    }

    // println!("l: {}, r: {}", l, r);
    println!(
        "{}",
        [
            st[..l].iter().collect::<String>(),
            st[l + 1..r].iter().collect::<String>(),
            st[l].to_string(),
            st[r..].iter().collect::<String>(),
        ]
        .join("")
    );
}
