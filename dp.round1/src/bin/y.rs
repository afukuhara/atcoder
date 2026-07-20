use proconio::{fastout, input, marker::Usize1};

const MOD: usize = 1_000_000_007;

struct Combination {
    fact: Vec<usize>,
    fact_inv: Vec<usize>,
}

impl Combination {
    fn new(max_n: usize) -> Self {
        let mut fact = vec![0; max_n + 1];
        let mut fact_inv = vec![0; max_n + 1];

        fact[0] = 1;

        for i in 1..=max_n {
            fact[i] = fact[i - 1] * i % MOD;
        }

        // 逆元を計算する（フェルマーの小定理を利用）
        fact_inv[max_n] = Self::power(fact[max_n], MOD - 2);

        for i in (0..max_n).rev() {
            fact_inv[i] = fact_inv[i + 1] * (i + 1) % MOD;
        }

        Self { fact, fact_inv }
    }

    fn choose(&self, n: usize, r: usize) -> usize {
        if r > n {
            return 0;
        }

        self.fact[n] * self.fact_inv[r] % MOD * self.fact_inv[n - r] % MOD
    }

    fn power(mut base: usize, mut exp: usize) -> usize {
        let mut res = 1;
        base %= MOD;
        while exp > 0 {
            if exp % 2 == 1 {
                res = (res * base) % MOD;
            }
            base = (base * base) % MOD;
            exp /= 2;
        }
        res
    }
}

#[fastout]
fn main() {
    input! {
        h: usize, w: usize, n: usize,
        mut walls: [(Usize1, Usize1); n]
    };

    let max_n = h + w - 2;
    let comb = Combination::new(max_n);

    walls.push((h - 1, w - 1));
    walls.sort();
    let mut dp = vec![0; walls.len()];

    for i in 0..walls.len() {
        let (yi, xi) = walls[i];

        // 障害物を無視した経路数
        dp[i] = comb.choose(yi + xi, yi);

        // 過去の壁を踏んでしまう経路を引く
        for j in 0..i {
            let (yj, xj) = walls[j];
            // println!("    prev wall: {}, {}", yj, xj);

            if yj <= yi && xj <= xi {
                let dy = yi - yj;
                let dx = xi - xj;
                let sub = dp[j] * comb.choose(dy + dx, dx) % MOD;
                dp[i] = (dp[i] + MOD - sub) % MOD;
            }
        }
    }

    println!("{}", dp[n]);
}
