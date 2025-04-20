use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize, k: usize,
    };

    let mut tree = vec![vec![]; n * k];

    for _ in 0..(n * k - 1) {
        input! {
            u: usize, v: usize,
        };

        tree[u - 1].push(v - 1);
        tree[v - 1].push(u - 1);
    }

    let mut stack: Vec<(usize, usize, usize)> = vec![];
    stack.push((0, 0, 0));
    let mut sizes = vec![1; n * k];

    let (dir_going, dir_back) = (0, 1);

    while let Some((node, parent, direction)) = stack.pop() {
        if direction == dir_going {
            stack.push((node, parent, dir_back));
            for &to_node in tree[node].iter().filter(|&&u| u != parent) {
                stack.push((to_node, node, dir_going));
            }
        } else {
            let mut count = 0;
            for &to_node in tree[node].iter().filter(|&&u| u != parent) {
                sizes[node] += sizes[to_node];
                if sizes[to_node] > 0 {
                    count += 1;
                }
            }

            // 一つの連鎖が k頂点を超えた
            if sizes[node] > k
            // 未完成の鎖が 3 本以上、頂点 v に集まった
            || count >= 3
            // 親に渡す途中で 2本以上の鎖が合流した 
            || (sizes[node] < k && count >= 2)
            {
                println!("No");
                return;
            }

            if sizes[node] == k {
                sizes[node] = 0;
            }
        }
    }

    println!("Yes");
}
