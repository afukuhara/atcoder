use proconio::{fastout, input};

const MOD: usize = 1_000_000_007;

#[fastout]
fn main() {
    input! {
        n: usize, mut k: u128,
        edges: [[usize; n]; n]
    };

    let mut base = edges.clone();

    // 単位行列
    let mut ans = vec![vec![0; n]; n];
    for i in 0..n {
        ans[i][i] = 1;
    }

    while k > 0 {
        if k % 2 == 1 {
            ans = multiply_matrix(&ans, &base);
        }
        base = multiply_matrix(&base, &base);
        k /= 2;
    }

    let mut total = 0;
    for i in 0..n {
        for j in 0..n {
            total += ans[i][j];
            total %= MOD;
        }
    }
    println!("{}", total);
}

fn multiply_matrix(a: &[Vec<usize>], b: &[Vec<usize>]) -> Vec<Vec<usize>> {
    let n = a.len();
    let mut c = vec![vec![0; n]; n];
    for i in 0..n {
        for j in 0..n {
            for k in 0..n {
                c[i][j] += a[i][k] * b[k][j];
                c[i][j] %= MOD;
            }
        }
    }
    c
}
