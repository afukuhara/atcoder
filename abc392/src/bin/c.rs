use itertools::Itertools;
use proconio::{fastout, input};

struct Person {
    watching_person: usize,
    bibs: usize,
}

impl Person {
    fn new(watching_person: usize, bibs: usize) -> Self {
        Self {
            watching_person,
            bibs,
        }
    }

    fn watching_person(&self) -> usize {
        self.watching_person
    }

    fn bibs(&self) -> usize {
        self.bibs
    }
}

#[fastout]
fn main() {
    input! {
        n: usize,
    };

    let mut people = vec![];
    for _ in 0..n {
        input! {
            p: usize,
        }

        people.push(Person::new(p, 0));
    }

    let mut bibs = vec![0; n];
    for i in 0..n {
        input! {
            q: usize,
        }

        people[i].bibs = q;
        bibs[q - 1] = people[i].watching_person - 1;
    }

    let ans = bibs.iter().map(|bib| people[*bib].bibs).join(" ");

    println!("{}", ans);
}
