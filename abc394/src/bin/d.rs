use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        s: String,
    };

    let mut begin_stack = Vec::new();
    let mut end_stack = Vec::new();

    for char in s.chars() {
        match char {
            '(' | '[' | '<' => begin_stack.push(char),
            _ => {
                end_stack.push(char);

                if let Some(begin) = begin_stack.pop() {
                    let end = end_stack.pop().unwrap();

                    if !pair_parentheses(begin, end) {
                        begin_stack.push(begin);
                        end_stack.push(end);
                        break;
                    }
                }
            }
        }
    }

    println!(
        "{}",
        if begin_stack.is_empty() && end_stack.is_empty() {
            "Yes"
        } else {
            "No"
        }
    );
}

fn pair_parentheses(first: char, second: char) -> bool {
    matches!((first, second), ('(', ')') | ('[', ']') | ('<', '>'))
}
