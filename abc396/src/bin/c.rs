use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize, m: usize,
        mut blacks: [isize; n],
        mut whites: [isize; m],
    };

    blacks.sort_by(|a, b| b.cmp(a));
    whites.sort_by(|a, b| b.cmp(a));

    let mut ans = 0;
    let mut is_both = true;

    for i in 0..n {
        let tmp_ans_black = ans + blacks[i];

        let tmp_ans_both = if i < whites.len() && whites[i] > 0 {
            tmp_ans_black + whites[i]
        } else {
            tmp_ans_black
        };

        if ans > tmp_ans_both {
            is_both = false;
        }

        let tmp_ans = if is_both { tmp_ans_both } else { tmp_ans_black };

        if ans >= tmp_ans {
            break;
        }

        ans = tmp_ans;
    }

    println!("{}", ans);
}
