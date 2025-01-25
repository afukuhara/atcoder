#[allow(unused_imports)]
use itertools::{iproduct, Itertools};
#[allow(unused_imports)]
use num_traits::pow;
#[allow(unused_imports)]
use proconio::{
    fastout, input,
    marker::{Chars, Usize1},
};
#[allow(unused_imports)]
use std::cmp::{max, min};
#[allow(unused_imports)]
use std::collections::{HashMap, HashSet, VecDeque};
#[allow(unused_imports)]
use std::iter::FromIterator;

#[fastout]
fn main() {
    input! {
        h: usize, w: usize,
    };

    let mut s = Vec::with_capacity(h);
    for _ in 0..h {
        input! { row: Chars };
        s.push(row);
    }

    let (mut si, mut sj, mut gi, mut gj) = (0, 0, 0, 0);
    for (i, chars) in s.iter().enumerate() {
        for (j, c) in chars.iter().enumerate() {
            if *c == 'S' {
                si = i;
                sj = j;
            } else if *c == 'G' {
                gi = i;
                gj = j;
            }
        }
    }

    // 無限大相当の定数と結果の初期値
    let inf = 1_000_000_000;
    let mut ans = inf;

    // moves[0] は「左右への移動」、moves[1] は「上下への移動」というセット
    let moves = [
        vec![(0, 1), (0, -1)], // 左右
        vec![(1, 0), (-1, 0)], // 上下
    ];

    // p = 0 と p = 1 の2パターンの BFS を行い、その最小手数を求める
    // p = 0 → 「黒から白は縦移動・白から黒は横移動」か、
    //          あるいはその逆パターンを表す (実装で (i + j + p) & 1 を使う)
    // p = 1 → もう一方の移動条件
    for p in 0..2 {
        // 各マスまでの最短距離を inf (未到達) で初期化
        let mut dist = vec![vec![inf; w]; h];
        dist[si][sj] = 0;

        // 幅優先探索 (BFS) のキュー
        let mut queue = VecDeque::new();
        queue.push_back((si, sj));

        while let Some((i, j)) = queue.pop_front() {
            // (i + j + p) & 1 の値に応じて moves[0] か moves[1] を使う
            let move_set = &moves[(i + j + p) & 1];
            for &(di, dj) in move_set {
                let ni = i as isize + di;
                let nj = j as isize + dj;
                // 範囲外はスキップ
                if ni < 0 || ni >= h as isize || nj < 0 || nj >= w as isize {
                    continue;
                }
                let (ni, nj) = (ni as usize, nj as usize);
                // 壁('#')ならスキップ
                if s[ni][nj] == '#' {
                    continue;
                }
                // 未到達なら更新
                if dist[ni][nj] == inf {
                    dist[ni][nj] = dist[i][j] + 1;
                    queue.push_back((ni, nj));
                }
            }
        }
        ans = ans.min(dist[gi][gj]);
    }

    // ans が更新されなければ -1 を出力 (到達不可能)
    if ans == inf {
        ans = -1;
    }
    println!("{}", ans);
}
