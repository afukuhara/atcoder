use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize, m: usize,
    };

    let mut walls = vec![0; n];

    for _ in 0..m {
        input! {
            l: usize, r: usize,
        };

        walls[l - 1] += 1;
        if r < n {
            walls[r] -= 1;
        }
    }

    let mut block = 0;
    let mut block_min = 1000000000;
    for wall in walls {
        block += wall;
        block_min = block_min.min(block);
    }

    println!("{}", block_min);
}
