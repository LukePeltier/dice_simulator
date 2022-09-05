pub mod die;

use crate::dice_box::die::Die;

pub struct DiceBox {
    dice: Vec<Die>,
    bonus: u8
}

impl DiceBox {
    pub fn new(dice: Vec<Die>, bonus: u8) -> DiceBox {
        DiceBox {dice, bonus}
    }

    pub fn from_string(input: String) -> DiceBox {
        let dice_box = DiceBox { dice: Vec::new(), bonus: 0};
        for ch in input.chars() {

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
}