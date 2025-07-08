use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
    };

    let mut source_grid = vec![vec![' '; n]; n];
    for row in 0..n {
        input! {
            s: String,
        }
        source_grid[row] = s.chars().collect();
    }

    let mut goal_grid = vec![vec![' '; n]; n];
    for row in 0..n {
        input! {
            s: String,
        }
        goal_grid[row] = s.chars().collect();
    }

    let transposed_source_grid_90 = transpose(&source_grid);
    let transposed_source_grid_180 = transpose(&transposed_source_grid_90);
    let transposed_source_grid_270 = transpose(&transposed_source_grid_180);

    let diffs = [
        compare_grid(&source_grid, &goal_grid),
        compare_grid(&transposed_source_grid_90, &goal_grid) + 1,
        compare_grid(&transposed_source_grid_180, &goal_grid) + 2,
        compare_grid(&transposed_source_grid_270, &goal_grid) + 3,
    ];

    let ans = diffs.iter().min().unwrap();
    println!("{}", ans);
}

fn transpose(grid: &[Vec<char>]) -> Vec<Vec<char>> {
    let mut transposed = vec![vec![' '; grid.len()]; grid[0].len()];
    let n = grid.len();
    for row in 0..n {
        for col in 0..n {
            transposed[col][n - row - 1] = grid[row][col];
        }
    }
    transposed
}

fn compare_grid(source_grid: &[Vec<char>], goal_grid: &[Vec<char>]) -> usize {
    let mut ans = 0;
    for row in 0..source_grid.len() {
        for col in 0..source_grid[0].len() {
            if source_grid[row][col] != goal_grid[row][col] {
                ans += 1;
            }
        }
    }
    ans
}
