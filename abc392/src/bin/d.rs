use std::fmt;
use std::fmt::Debug;

use num_rational::Rational64;
use num_traits::ToPrimitive;

use itertools::Itertools;
use proconio::{fastout, input};
use std::collections::HashMap;

struct Dice {
    dice_numbers: HashMap<usize, usize>,
    len: usize,
}

impl Debug for Dice {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Dice")
            .field("dice_numbers", &self.dice_numbers)
            .field("len", &self.len)
            .finish()
    }
}

impl Dice {
    fn new() -> Self {
        let dice_numbers = HashMap::new();
        Self {
            dice_numbers,
            len: 0,
        }
    }

    fn add(&mut self, num: usize) {
        *self.dice_numbers.entry(num).or_insert(0) += 1;
        self.len += 1;
    }

    fn number_of_dice(&self) -> usize {
        self.dice_numbers.len()
    }

    fn get_probability(&self, num: usize) -> Rational64 {
        if let Some(count) = self.dice_numbers.get(&num) {
            Rational64::new(*count as i64, self.len as i64)
        } else {
            Rational64::new(0, 1)
        }
    }

    fn calculate_combination_probability(&self, other: &Self) -> Rational64 {
        let (larger_dice, smaller_dice) = if self.number_of_dice() > other.number_of_dice() {
            (self, other)
        } else {
            (other, self)
        };

        let mut probability = Rational64::new(0, 1);
        for (num, _count) in smaller_dice.dice_numbers.iter() {
            probability += smaller_dice.get_probability(*num) * larger_dice.get_probability(*num);
        }
        probability
    }
}

#[fastout]
fn main() {
    input! {
        n: usize
    };

    let mut dices = vec![];
    for _ in 0..n {
        input! {
            k: i64,
        }

        let mut dice = Dice::new();
        for _ in 0..k {
            input! {
                a: usize,
            }
            dice.add(a);
        }
        dices.push(dice);
    }

    let mut max_probability = Rational64::new(0, 1);
    for comb in dices.iter().combinations(2) {
        let probability = comb[0].calculate_combination_probability(comb[1]);
        if probability > max_probability {
            max_probability = probability;
        }
    }

    println!("{}", max_probability.to_f64().unwrap());
}
