use proconio::{
    fastout, input,
    marker::Usize1,
};
use std::collections::VecDeque;

const W: usize = 1 << 10;

#[fastout]
fn main() {
    input! {
        n: usize, m: usize,
        edges: [(Usize1, Usize1, usize); m],
    };

    let mut graph = vec![Vec::<(usize, usize)>::new(); n];
    for (a, b, w) in edges {
        graph[a].push((b, w));
    }

    let mut visited = vec![false; n * W];
    let mut queue = VecDeque::new();

    push(0, 0, &mut visited, &mut queue);

    while let Some(id) = queue.pop_front() {
        let (v, x) = (id / W, id % W);
        for &(to, w) in &graph[v] {
            push(to, x ^ w, &mut visited, &mut queue);
        }
    }

    for x in 0..W {
        if visited[(n - 1) * W + x] {
            println!("{}", x);
            return;
        }
    }
    println!("-1");
}

fn push(v: usize, x: usize, visited: &mut [bool], q: &mut VecDeque<usize>) {
    let id = v * W + x;
    if !visited[id] {
        visited[id] = true;
        q.push_back(id);
    }
}
