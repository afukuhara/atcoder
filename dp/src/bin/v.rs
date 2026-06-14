use proconio::{fastout, input, marker::Usize1};

#[fastout]
fn main() {
    input! {
        n: usize, m: usize,
        mut edges: [(Usize1, Usize1); n - 1]
    };

    let mut graph = vec![vec![]; n];

    for (a, b) in edges {
        graph[a].push(b);
        graph[b].push(a);
    }

    let root = 0;
    let mut parent = vec![usize::MAX; n];
    let mut order = Vec::with_capacity(n);

    // 木を root = 0 で根付木にする
    // ------------------------------
    parent[root] = root;
    let mut stack = vec![root];

    while let Some(v) = stack.pop() {
        order.push(v);

        for &to in &graph[v] {
            if parent[to] == usize::MAX {
                parent[to] = v;
                stack.push(to);
            }
        }
    }

    // down[v]
    // vからみて子側だけを使って、v を含む連結な黒集合を作る通りの数
    // ------------------------------
    let mut down = vec![1; n];
    for &v in order.iter().rev() {
        let mut prod = 1;

        for &to in graph[v].iter().filter(|&&to| parent[to] == v) {
            prod = (prod * (down[to] + 1)) % m;
        }

        down[v] = prod;
    }

    // up[v]
    // vからみて親側を使って、v に接続できる黒い連結部分の通り数
    // ------------------------------
    let mut up = vec![0; n]; // root には親側がないので 0 で初期化
    let mut ans = vec![0; n];

    for &v in &order {
        let deg = graph[v].len(); // 頂点 v に隣接する頂点の数

        // vals[i] = graph[v][i] 方向から v に来る contribution
        let mut vals = vec![0; deg];

        for i in 0..deg {
            let to = graph[v][i];
            vals[i] = if to == parent[v] && v != root {
                up[v] // 親方向から来る値
            } else {
                down[to] // 子方向から来る値
            }
        }

        // prefix[i] = 0..i まで (= i個分の積)
        let mut prefix = vec![1; deg + 1];
        for i in 0..deg {
            prefix[i + 1] = prefix[i] * ((vals[i] + 1) % m) % m;
        }

        // suffix[i] = i..deg まで (= (deg - i)個分の積)
        let mut suffix = vec![1; deg + 1];
        for i in (0..deg).rev() {
            suffix[i] = suffix[i + 1] * ((vals[i] + 1) % m) % m;
        }

        // v を黒にする通りの数
        ans[v] = prefix[deg];

        // 子に up を渡す
        for i in 0..deg {
            let to = graph[v][i];
            if parent[to] == v {
                // to 方向だけを除いた積
                // これは contribution[v -> to] になる
                up[to] = prefix[i] * suffix[i + 1] % m;
            }
        }
    }

    for v in 0..n {
        println!("{}", ans[v]);
    }
}
