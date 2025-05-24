use proconio::{fastout, input, marker::Chars};
use std::collections::VecDeque;

#[fastout]
fn main() {
    input! {
        h: usize, w: usize,
    };

    // グリッドの読み込み
    let mut s = Vec::new();
    for _ in 0..h {
        input! { c: Chars }
        s.push(c);
    }

    input! {
        a: usize, b: usize, c: usize, d: usize,
    }

    let (a, b, c, d) = (a - 1, b - 1, c - 1, d - 1);

    // 距離配列の初期化
    let mut ans = vec![vec![h * w; w]; h];
    let mut dq = VecDeque::new();

    // 方向ベクトル
    let dx = [-1, 1, 0, 0];
    let dy = [0, 0, -1, 1];

    // 開始地点の設定
    ans[a][b] = 0;
    dq.push_front((a, b));

    while let Some((x, y)) = dq.pop_front() {
        // 目標地点に到達した場合
        if x == c && y == d {
            println!("{}", ans[c][d]);
            return;
        }

        // 4方向への移動を試行
        for i in 0..4 {
            // 1歩目と2歩目を調べる
            for j in 1..=2 {
                let nx = x as i32 + dx[i] * j;
                let ny = y as i32 + dy[i] * j;

                // 範囲外チェック
                if nx < 0 || nx >= h as i32 || ny < 0 || ny >= w as i32 {
                    break;
                }

                let nx = nx as usize;
                let ny = ny as usize;

                // 壁のチェック
                if s[nx][ny] == '#' {
                    // 壁がある場合（壁を破って進む）
                    if ans[nx][ny] > ans[x][y] + 1 {
                        ans[nx][ny] = ans[x][y] + 1;
                        dq.push_back((nx, ny));
                    }
                } else {
                    // 壁がない場合（1歩目のみ）
                    if j == 1 && ans[nx][ny] > ans[x][y] {
                        ans[nx][ny] = ans[x][y];
                        dq.push_front((nx, ny));
                    }
                }
            }
        }
    }
}
