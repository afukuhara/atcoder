use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        q: usize
    };

    let mut queries = vec![];
    for _ in 0..q {
        input! {
            a: usize
        };

        if a == 1 || a == 3 {
            input! {
                b: usize
            };
            queries.push((a, b));
        } else {
            queries.push((a, 0));
        }
    }

    let mut sum_at_index = vec![];
    let mut offset = 0;
    for (i, j) in &queries {
        match i {
            1 => {
                let new_sum = match sum_at_index.last() {
                    Some(sum) => sum + *j,
                    None => *j,
                };
                sum_at_index.push(new_sum);
            }
            2 => {
                offset += 1;
            }
            3 => {
                let count = if *j > 0 { *j - 1 } else { 0 };

                if count == 0 {
                    println!("0");
                    continue;
                }

                let sum = if offset == 0 {
                    sum_at_index[offset + count - 1]
                } else {
                    sum_at_index[offset + count - 1] - sum_at_index[offset - 1]
                };
                println!("{}", sum);
            }
            _ => unreachable!(),
        }
    }
}
