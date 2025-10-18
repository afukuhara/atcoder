#[allow(unused_imports)]
use itertools::Itertools;
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        _n: usize, q: usize,
    };

    let mut queries: Vec<(usize, usize, String)> = vec![];

    for _ in 0..q {
        input! { kind: usize, p: usize };
        if kind == 2 {
            input! { text: String };
            queries.push((kind, p, text));
        } else {
            queries.push((kind, p, String::new()));
        }
    }

    let mut pc = 0;
    let mut texts: Vec<String> = vec![];

    for (kind, p, text) in queries.iter().rev() {
        match kind {
            1 if *p == pc => {
                pc = 0;
            }
            2 if *p == pc => {
                texts.push(text.clone());
            }
            3 if pc == 0 => {
                pc = *p;
            }
            _ => continue,
        }
    }

    println!("{}", texts.iter().rev().join(""));
}

// fn main() {
//     input! {
//         n: usize, q: usize,
//     };

//     let mut server = String::new();
//     let mut pcs = vec![String::new(); n];

//     for _ in 0..q {
//         input! { query: usize };
//         match query {
//             1 => {
//                 input! { p: Usize1 };
//                 pcs[p] = server.clone();
//             }
//             2 => {
//                 input! { p: Usize1, text: String };
//                 pcs[p].push_str(&text);
//             }
//             3 => {
//                 input! { p: Usize1 };
//                 server = pcs[p].clone();
//             }
//             _ => unreachable!(),
//         }
//     }

//     println!("{}", server);
// }
