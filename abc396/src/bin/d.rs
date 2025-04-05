use proconio::{fastout, input};
use std::collections::{HashMap, HashSet};

#[fastout]
fn main() {
    input! {
        n: usize, m: usize,
    };

    let mut edges: HashMap<usize, Vec<(usize, usize)>> = HashMap::new();

    for _ in 0..m {
        input! {
            u: usize, v: usize, w: usize,
        };

        edges.entry(u).or_default().push((v, w));
        edges.entry(v).or_default().push((u, w));
    }

    let mut answers = Vec::new();
    let mut visited = HashSet::new();
    walkthrough(&edges, n, 1, &mut answers, 0, &mut visited);

    println!("{:?}", answers.iter().min().unwrap());
}

fn walkthrough(
    edges: &HashMap<usize, Vec<(usize, usize)>>,
    last_edge: usize,
    current_edge: usize,
    answers: &mut Vec<usize>,
    tmp_ans: usize,
    visited: &mut HashSet<usize>,
) {
    for (v, w) in edges[&current_edge]
        .iter()
        .filter(|(v, _)| !visited.contains(v))
        .cloned()
    {
        let new_tmp_ans = tmp_ans ^ w;
        let mut new_visited = visited.clone();
        new_visited.insert(current_edge);

        if v == last_edge {
            answers.push(new_tmp_ans);
            continue;
        } else if !edges.contains_key(&v) {
            continue;
        }

        walkthrough(edges, last_edge, v, answers, new_tmp_ans, &mut new_visited);
    }
}
