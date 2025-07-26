use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        num_zoos: usize, num_animals: usize,
    };

    let mut entry_fees = vec![];

    for _ in 0..num_zoos {
        input! { c: usize };
        entry_fees.push(c);
    }

    let mut animal_zoos = vec![vec![]; num_animals];

    for i in 0..num_animals {
        input! { k: usize };
        animal_zoos[i] = vec![0; k];
        for j in 0..k {
            input! { a: usize };
            animal_zoos[i][j] = a - 1;
        }
    }

    let mut pow3 = vec![1; num_zoos + 1];
    for i in 0..num_zoos {
        pow3[i + 1] = pow3[i] * 3;
    }

    const INF: usize = 1_000_000_000_000_000_000;
    let mut min_cost = INF;

    for state in 0..pow3[num_zoos] {

        // 3 進数の各桁 → 動物園ごとの訪問回数 (0,1,2)
        let mut visit_counts = vec![0; num_zoos];
        for i in 0..num_zoos {
            visit_counts[i] = (state / pow3[i]) % 3;
        }

        // 入園料の合計を計算
        let mut cost = 0;
        for i in 0..num_zoos {
            cost += entry_fees[i] * visit_counts[i];
        }

        // 既に最小値を超えるならスキップ
        if cost >= min_cost {
            continue;
        }

        // 各動物が２回以上見られるか確認
        let mut is_valid = true;
        for animal in 0..num_animals {
            let mut view_count = 0;
            for &zoo in &animal_zoos[animal] {
                view_count += visit_counts[zoo];
            }
            if view_count < 2 {
                is_valid = false;
            }

            if !is_valid {
                break;
            }
        }

        if is_valid {
            min_cost = cost;
        }
    }

    println!("{:?}", min_cost);
}
