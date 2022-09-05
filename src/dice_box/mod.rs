pub mod die;

use std::collections::BTreeSet;

use crate::dice_box::die::Die;
use regex::Regex;

pub struct DiceBox {
    dice: Vec<Die>,
    bonus: u8,
}

impl DiceBox {
    #[allow(dead_code)]
    pub fn new(dice: Vec<Die>, bonus: u8) -> DiceBox {
        DiceBox { dice, bonus }
    }

    pub fn from_string(input: String) -> DiceBox {
        let items = input.split("+");
        let mut dice_box = DiceBox {
            dice: Vec::new(),
            bonus: 0,
        };
        for item in items {
            let trimmed_item = item.trim();

            let dice_regex = match Regex::new(r"^([0-9]+)d([0-9]+)$") {
                Ok(result) => result,
                Err(err) => panic!("Error compiling regex statement, {}", err),
            };

            if dice_regex.is_match(trimmed_item) {
                let dice_captures = match dice_regex.captures(trimmed_item) {
                    Some(result) => result,
                    None => panic!(
                        "Could not match string {} with regex statement.",
                        trimmed_item
                    ),
                };
                let die_num_match = match dice_captures.get(1) {
                    Some(result) => result,
                    None => panic!(
                        "Could not capture number of dice in string {}",
                        trimmed_item
                    ),
                };

                let die_num: u8 = match die_num_match.as_str().parse::<u8>() {
                    Ok(result) => result,
                    Err(err) => panic!(
                        "Error turning die num match {} into num, {}",
                        die_num_match.as_str(),
                        err
                    ),
                };

                let die_sides_match = match dice_captures.get(2) {
                    Some(result) => result,
                    None => panic!(
                        "Could not capture sid
            children_boxes: None,es of dice in string {}",
                        trimmed_item
                    ),
                };

                let die_sides: u8 = match die_sides_match.as_str().parse::<u8>() {
                    Ok(result) => result,
                    Err(err) => panic!(
                        "Error turning die sides match {} into num, {}",
                        die_num_match.as_str(),
                        err
                    ),
                };

                for _ in 0..die_num {
                    let die_to_add = Die::new(1, die_sides, 0);
                    dice_box.dice.push(die_to_add);
                }

                continue;
            }

            let raw_num: Option<u8> = match trimmed_item.parse::<u8>() {
                Ok(result) => Some(result),
                Err(_) => None,
            };
            if raw_num.is_some() {
                dice_box.bonus += raw_num.unwrap();
                continue;
            } else {
                panic!("Unable to parse item {} in input {}", trimmed_item, input);
            }
        }
        return dice_box;
    }

    pub fn roll_dice(&self) -> u32 {
        let mut results: u32 = 0;
        for die in &self.dice {
            results += die.roll() as u32;
        }
        return results + self.bonus as u32;
    }

    pub fn get_lowest(&self) -> u32 {
        let mut results: u32 = 0;
        for die in &self.dice {
            results += die.get_lowest() as u32;
        }
        return results + self.bonus as u32;
    }

    pub fn get_highest(&self) -> u32 {
        let mut results: u32 = 0;
        for die in &self.dice {
            results += die.get_highest() as u32;
        }
        return results + self.bonus as u32;
    }

    pub fn get_all_outcomes(&self) -> BTreeSet<u32> {
        let dice_len = self.dice.len();
        if dice_len <= 0 {
            return BTreeSet::new();
        }
        let mut possible_outcomes: BTreeSet<u32> = BTreeSet::new();
        let first_die_outcomes = self.dice[0].get_outcomes();

        let recursive_slice = &self.dice[1..];

        if recursive_slice.is_empty() {
            //BTreeSet is just current outcomes
            for outcome in first_die_outcomes {
                possible_outcomes.insert(outcome as u32);
            }
            return possible_outcomes;
        }

        let recursive_dice_box = DiceBox {
            dice: recursive_slice.to_vec(),
            bonus: 0,
        };

        let all_other_outcomes = recursive_dice_box.get_all_outcomes();

        for outcome in first_die_outcomes {
            for other_outcome in &all_other_outcomes {
                possible_outcomes.insert(outcome as u32 + other_outcome + self.bonus as u32);
            }
        }

        return possible_outcomes;
    }

    pub fn get_mean(&self) -> f32{
        let mut results: f32 = 0.0;
        for die in &self.dice {
            results += die.get_mean();
        }
        return results + self.bonus as f32;
    }
}
