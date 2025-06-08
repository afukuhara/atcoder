use proconio::{fastout, input};
use std::collections::HashSet;

#[fastout]
fn main() {
    input! {
        n: usize, m: usize,
    };

    let mut menu_list = Vec::new();
    let mut ingigredient_list = vec![HashSet::new(); n + 1];

    for i in 0..m {
        input! { k: usize }

        let mut menu = HashSet::new();
        for _ in 0..k {
            input! { a: usize }

            menu.insert(a);
            ingigredient_list[a].insert(i + 1);
        }

        menu_list.push(menu);
    }

    let mut ans = 0;
    for _ in 0..n {
        input! { b: usize }

        for menu_num in ingigredient_list[b].iter() {
            let menu_num = *menu_num - 1;
            menu_list[menu_num].remove(&b);

            if menu_list[menu_num].is_empty() {
                ans += 1;
            }
        }

        println!("{}", ans);
    }
}
