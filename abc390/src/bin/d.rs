use proconio::{fastout, input};
use std::collections::HashSet;

#[fastout]
fn main() {
    input! {
        n: usize,
        stone_bags: [usize; n]
    };

    let mut bag_groups: Vec<Vec<usize>> = Vec::new();
    let mut results: HashSet<usize> = HashSet::new();
    // 袋0（袋1に相当）から探索開始
    another_dfs(0, &stone_bags, &mut bag_groups, &mut results);

    println!("{:?}", results.len());
}

/// 深さ優先探索でグループ分け（パーティション）を全探索する関数
///
/// # 引数
/// - `bag_index`: 現在処理する袋の番号（深さ d に相当）
/// - `bags`: 各袋の石の数のリスト
/// - `bag_groups`: 現在のグループ分け状態。各グループは袋のインデックスのベクター
/// - `results`: 完成したグループ分けに対して計算した XOR の値の結果を保存するリスト
fn another_dfs(
    bag_index: usize,
    bags: &Vec<usize>,
    bag_groups: &mut Vec<Vec<usize>>,
    results: &mut HashSet<usize>,
) {
    // すべての袋が処理された場合（深さが N になった場合）
    if bag_index == bags.len() {
        // 各グループごとに、袋に入っている石の数の合計を計算し、
        // その XOR を求める
        let mut xor_value = 0;
        for group in bag_groups.iter() {
            let group_sum: usize = group.iter().map(|&i| bags[i]).sum();
            xor_value ^= group_sum;
        }
        results.insert(xor_value);
        return;
    }

    // 現在のグループ数（k）
    let k = bag_groups.len();

    // 【既存のグループに追加する選択肢】
    // もし既存のグループがあれば、各グループに対して袋 bag_index を追加する
    for i in 0..k {
        bag_groups[i].push(bag_index);
        // 次の袋へ進む（深さ d+1）
        another_dfs(bag_index + 1, bags, bag_groups, results);
        // 探索終了後は元に戻す（バックトラッキング）
        bag_groups[i].pop();
    }

    // 【新しいグループを作る選択肢】
    // ここでは、袋 bag_index のみが所属する新たなグループを作成する
    bag_groups.push(vec![bag_index]);
    another_dfs(bag_index + 1, bags, bag_groups, results);
    bag_groups.pop();
}
