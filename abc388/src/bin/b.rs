use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: u32, d: u32,
    };

    let mut snakes = vec![];
    for _i in 0..n {
        input! { t: u32, l: u32 }
        snakes.push((t, l));
    }

    for i in 1..=d {
        let result = heaviest_snake(&snakes, i);
        println!("{}", result);
    }
}

fn heaviest_snake(snakes: &[(u32, u32)], n: u32) -> u32 {
    snakes.iter().map(|(t, l)| t * (l + n)).max().unwrap()
}
