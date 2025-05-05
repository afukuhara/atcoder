use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize, m: usize,
    };

    let mut graph = vec![vec![]; n];
    for _ in 0..m {
        input! {
            mut a: usize, mut b: usize,
        };
        (a, b) = (a - 1, b - 1);
        graph[a].push(b);
        graph[b].push(a);
    }

    let num_of_connected_components = count_connected_components(&graph);
    println!("{}", m - (n - num_of_connected_components));
}

fn count_connected_components(adj: &[Vec<usize>]) -> usize {
    let n = adj.len();
    let mut visited = vec![false; n];
    let mut count = 0;

    for i in 0..n {
        if !visited[i] {
            dfs(i, adj, &mut visited);
            count += 1;
        }
    }

    count
}

fn dfs(v: usize, adj: &[Vec<usize>], visited: &mut [bool]) {
    visited[v] = true;
    for &to in &adj[v] {
        if !visited[to] {
            dfs(to, adj, visited);
        }
    }
}
