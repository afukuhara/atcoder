use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        mut s: String
    };

    let mut number_of_a = s
        .chars()
        .map(|c| c.to_digit(10).unwrap())
        .collect::<Vec<_>>();

    let mut total_count = 0;
    let mut b_count = 0;

    while let Some(first_digit) = number_of_a.pop() {
        let digit_after_click = ((10 + first_digit) - (b_count % 10)) % 10;
        b_count += digit_after_click;
        total_count += digit_after_click + 1;
    }

    println!("{}", total_count);
}

/*
S: 21

#1, A -> 0
#2, B -> 1
#3, A -> 10
#4, B -> 21

S: 407

#-0, A -> 407
#-1, A -> 396
#-2, A -> 285
#-3, A -> 174
#-4, A -> 063
#-5, A -> 952
#-6, A -> 841
#-7, A -> 730
#-8, B -> 73
#-9, A -> 62
#-10, B -> 51
#-11, A -> 40
#-12, B -> 4
#-13, A -> 3
#-14, A -> 2
#-15, A -> 1
#-16, A -> 0
#-17, A ->

*/
