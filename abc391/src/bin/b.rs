use proconio::{fastout, input, marker::Chars};

#[fastout]
fn main() {
    input! {
        n: usize, m: usize,
    };

    let mut s = Vec::new();
    for _ in 0..n {
        input! {
            a: Chars,
        }
        s.push(a);
    }

    let mut t = Vec::new();
    for _ in 0..m {
        input! {
            b: Chars,
        }
        t.push(b);
    }

    for i in 0..n {
        let positions = find_subsequence(&s[i], &t[0]);

        if positions.is_empty() {
            continue;
        }

        for pos in positions {
            let mut all_match = true;
            for idx in 1..m {
                let next_s = &s[i + idx][pos..pos + t[idx].len()];
                let next_t = &t[idx];

                if next_s != next_t {
                    all_match = false;
                    break;
                }
            }
            if all_match {
                println!("{} {}", i + 1, pos + 1);
                return;
            }
        }
    }
}

fn find_subsequence<T: PartialEq>(a: &[T], b: &[T]) -> Vec<usize> {
    // b の長さのスライスを A 全体から順に取り出し、
    // そのスライスが B と一致するかをチェックする
    // 該当するすべての位置を返す
    a.windows(b.len())
        .enumerate()
        .filter(|(_, window)| window == &b)
        .map(|(i, _)| i)
        .collect()
}
