pub use rand::prelude::*;

#[derive(Clone)]
pub struct Die {
    min_value: u8,
    max_value: u8,
    rerolls: u8
}

impl Die {
    pub fn new(min_value: u8, max_value: u8, rerolls: u8) -> Die {
        Die {
            min_value,
            max_value,
            rerolls
        }
    }

    pub fn roll(&self) -> u8 {
        let mut rng: ThreadRng = rand::thread_rng();
        match self.rerolls {
            0 => return rng.gen_range(self.min_value..=self.max_value),
            _ => {
                let mut best_result: Option<u8> = None;
                for _ in 0..self.rerolls {
                    let roll: u8 = rng.gen_range(self.min_value..=self.max_value);
                    if roll > best_result.unwrap_or(0) {
                        best_result = Some(roll);
                    }
                }
                return best_result.unwrap_or(0);
            }
        }
    }

    pub fn get_lowest(&self) -> u8 {
        self.min_value
    }

    pub fn get_highest(&self) -> u8 {
        self.max_value
    }

    pub fn get_outcomes(&self) -> Vec<u8> {
        let mut outcomes: Vec<u8> = vec![];
        for i in self.min_value..=self.max_value {
            outcomes.push(i);
        }
        return outcomes;
    }

    pub fn get_mean(&self) -> f32 {
        (self.max_value as f32 + self.min_value as f32) / 2.0
    }
}
