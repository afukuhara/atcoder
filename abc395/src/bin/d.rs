use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        q: usize,
    }

    let mut box_to_label = (0..n).collect::<Vec<_>>();
    let mut label_to_box = (0..n).collect::<Vec<_>>();
    let mut pigeon_to_box = (0..n).collect::<Vec<_>>();

    for _ in 0..q {
        input! {
            query: usize,
        }

        match query {
            1 => {
                input! {
                    pigeon: usize,
                    to: usize,
                }

                pigeon_to_box[pigeon - 1] = label_to_box[to - 1];
            }
            2 => {
                input! {
                    label0: usize,
                    label1: usize,
                }

                let label0 = label0 - 1;
                let label1 = label1 - 1;

                label_to_box.swap(label0, label1);

                let box0 = label_to_box[label0];
                let box1 = label_to_box[label1];
                box_to_label.swap(box0, box1);
            }
            3 => {
                input! {
                    pigeon: usize,
                }

                let pigeon = pigeon - 1;
                let box_id = pigeon_to_box[pigeon];
                let label = box_to_label[box_id];

                println!("{}", label + 1);
            }
            _ => unreachable!(),
        }
    }
}
