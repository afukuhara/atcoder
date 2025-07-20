use proconio::{fastout, input};
use std::collections::VecDeque;

// 「サイクルグラフである」ための必要十分条件（N ≥ 3）
// 1.  辺数と頂点数が等しい：M == N
// 2. 各頂点の次数がちょうど 2
// 3. グラフが連結

#[fastout]
fn main() {
    input! {
        n: usize, m: usize,
    };

    // 1.  辺数と頂点数が等しい：M == N
    if m != n {
        println!("No");
        return;
    }

    let mut adjacency_list = vec![vec![]; n + 1];
    let mut degrees = vec![0; n + 1];

    for _ in 0..m {
        input! {
            a: usize, b: usize,
        };
        degrees[a] += 1;
        degrees[b] += 1;
        adjacency_list[a].push(b);
        adjacency_list[b].push(a);
    }

    // 2. 各頂点の次数がちょうど 2
    if degrees.iter().skip(1).any(|&d| d != 2) {
        println!("No");
        return;
    }

    // 3. グラフが連結している (BFS)
    let mut queue = VecDeque::new();
    let mut visited = vec![false; n + 1];
    queue.push_back(1);
    visited[1] = true;

    let mut visited_count = 0;
    while let Some(current) = queue.pop_front() {
        visited_count += 1;
        for &neibor in &adjacency_list[current] {
            if !visited[neibor] {
                visited[neibor] = true;
                queue.push_back(neibor);
            }
        }
    }

    println!("{}", if visited_count == n { "Yes" } else { "No" });
}
