use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        nums: [String; 5],
    };

    let input_nums = nums.join(" ");
    let correct_answers = ["2 1 3 4 5", "1 3 2 4 5", "1 2 4 3 5", "1 2 3 5 4"];

    let ans = if correct_answers.iter().any(|answer| input_nums == *answer) {
        "Yes"
    } else {
        "No"
    };

    println!("{}", ans);
}
