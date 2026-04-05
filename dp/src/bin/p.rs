use proconio::{fastout, input, marker::Usize1};

#[fastout]
fn main() {
    input! {
        n: usize,
        mut edges: [(Usize1, Usize1); n-1]
    };

    const MOD: usize = 1_000_000_007;
    let mut graph = vec![vec![]; n];
    for (a, b) in edges {
        graph[a].push(b);
        graph[b].push(a);
    }

    let mut dp_white = vec![0; n];
    let mut dp_black = vec![0; n];
    let (white, black) = dfs(0, 0, &graph, &mut dp_white, &mut dp_black, MOD);

    println!("{}", (white + black) % MOD);
}

fn dfs(
    v: usize,
    parent: usize,
    graph: &[Vec<usize>],
    dp_white: &mut [usize],
    dp_black: &mut [usize],
    divider: usize,
) -> (usize, usize) {
    if dp_white[v] != 0 && dp_black[v] != 0 {
        return (dp_white[v], dp_black[v]);
    }

    let mut white = 1;
    let mut black = 1;

    for &nv in graph[v].iter().filter(|&nv| *nv != parent) {
        let (nw, nb) = dfs(nv, v, graph, dp_white, dp_black, divider);
        white = (white * (nw + nb)) % divider;
        black = (black * nw) % divider;
    }

    dp_white[v] = white;
    dp_black[v] = black;

    (white, black)
}
