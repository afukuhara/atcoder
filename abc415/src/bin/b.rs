use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        s: String,
    };

    // 自分のコード
    // let mut inventories = vec![];
    // for (i, c) in s.chars().enumerate() {
    //     if c == '#' {
    //         inventories.push(i + 1);
    //     }
    // }

    // for i in inventories.chunks(2) {
    //     println!("{},{}", i[0], i[1]);
    // }

    for i in s
        .chars()
        .enumerate()
        .filter_map(|(i, c)| (c == '#').then_some(i + 1))
        .collect::<Vec<_>>()
        .chunks(2)
    {
        println!("{},{}", i[0], i[1]);
    }
}
