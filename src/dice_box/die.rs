pub use rand::prelude::*;

pub struct Die {
    min_value: u8,
    max_value: u8,
    rerolls: u8,
    bonus: u8,
}

impl Die {
    pub fn new(min_value: u8, max_value: u8, rerolls: u8, bonus: u8) -> Die {
        Die {
            min_value,
            max_value,
            rerolls,
            bonus,
        }
    }

    pub fn roll(&self) -> u8 {
        let mut rng: ThreadRng = rand::thread_rng();
        match self.rerolls {
            0 => return rng.gen_range(self.min_value..=self.max_value) + self.bonus,
            _ => {
                let mut best_result: Option<u8> = None;
                for _ in 0..self.rerolls {
                    let roll: u8 = rng.gen_range(self.min_value..=self.max_value);
                    if roll > best_result.unwrap_or(0) {
                        best_result = Some(roll);
                    }
                }
                return best_result.unwrap_or(0) + self.bonus;
            }
        }
    }
}
