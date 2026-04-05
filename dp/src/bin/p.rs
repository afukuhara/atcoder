use proconio::{fastout, input, marker::Usize1};

#[fastout]
fn main() {
    input! {
        n: usize,
        mut edges: [(Usize1, Usize1); n-1]
    };

    let mut graph = vec![vec![]; n];
    for (a, b) in edges {
        graph[a].push(b);
        graph[b].push(a);
    }

    let (white, black) = dfs(0, n, &graph);

    println!("{}", (white + black) % MOD);
}

const MOD: usize = 1_000_000_007;
fn dfs(v: usize, parent: usize, graph: &[Vec<usize>]) -> (usize, usize) {
    let mut white = 1;
    let mut black = 1;

    for &nv in graph[v].iter().filter(|&nv| *nv != parent) {
        let (nw, nb) = dfs(nv, v, graph);

        // v が白なら子は白黒どちらでもよい
        white = (white * (nw + nb)) % MOD;

        // v が黒なら子は白のみ
        black = (black * nw) % MOD;
    }

    (white, black)
}
