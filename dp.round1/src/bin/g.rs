use proconio::{fastout, input, marker::Usize1};

#[fastout]
fn main() {
    input! {
        n: usize, m: usize,
        edges: [(Usize1, Usize1); m],
    };

    let mut graph = vec![vec![]; n];

    for (start, end) in edges {
        graph[start].push(end);
    }

    let mut dp = vec![None; n];

    let mut res = 0;
    for v in 0..n {
        res = res.max(rec(v, &graph, &mut dp));
    }

    println!("{}", res);
}

fn rec(v: usize, graph: &[Vec<usize>], dp: &mut [Option<usize>]) -> usize {
    if let Some(val) = dp[v] {
        return val;
    }

    let best = graph[v]
        .iter()
        .map(|&nv| rec(nv, graph, dp) + 1)
        .max()
        .unwrap_or(0);
    dp[v] = Some(best);
    best
}
