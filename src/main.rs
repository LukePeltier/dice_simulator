use rand::prelude::*;
use std::env;

struct Die {
    min_value: u8,
    max_value: u8,
    rerolls: u8,
    bonus: u8
}

impl Die {
    fn roll(&self) -> u8 {
        let mut rng: ThreadRng = rand::thread_rng();
        match self.rerolls {
            0 => return rng.gen_range(self.min_value..=self.max_value) + self.bonus,
            _ => {
                let mut best_result: Option<u8> = None;
                for _ in 0..self.rerolls {
                    let roll: u8 = rng.gen_range(self.min_value..=self.max_value);
                    if roll > best_result.unwrap_or(0){
                        best_result = Some(roll);
                    }
                }
                return best_result.unwrap_or(0) + self.bonus;
            }
        }
    }
}

struct DiceBox {
    dice: Vec<Die>,
    bonus: u8
}

impl DiceBox {
    fn from_string(input: String) -> DiceBox {
        let dice_box = DiceBox { dice: Vec::new(), bonus: 0};
        for ch in input.chars() {

        }
        return dice_box;
    }

    fn roll_dice(&self) -> u32 {
        let mut results: u32 = 0;
        for die in &self.dice {
            results += die.roll() as u32;
        }
        return results + self.bonus as u32;
    }
}


fn median(data: &Vec<u32>) -> u32 {
    let mut sorted: Vec<u32> = data.clone();

    sorted.sort();

    let middle = (((sorted.len()-1) as f32)/2.0).round() as usize;

    match sorted.get(middle) {
        Some(x) => *x,
        None => 0
    }
}

fn usage() {
    println!("Usage: dice_simulator [dice string] [iterations]")
}

fn main() {
    let mut iterations: f32 = 1000.0;
    let dice_string;
    let args: Vec<String> = env::args().collect();

    match args.len() {
        3 => {
            dice_string = args[1].clone();
            let received_iterations = args[2].clone();
            iterations = match received_iterations.parse::<f32>() {
                Ok(res) => { res },
                Err(_err) => {
                    println!("Could not find number, using default");
                    iterations
                }
            };
            println!("Received dice string: {}", dice_string);
            println!("Using  iterations: {}", iterations);
        },
        _ => {
            usage();
            return;
        }
    }
    // let main_box = DiceBox::from_string(dice_string);
    let main_box = DiceBox { dice: vec![Die{min_value: 1, max_value: 20, rerolls: 0, bonus: 0}], bonus: 0};
    let mut results: Vec<u32> = vec![];

    for _ in 0..iterations as u32{
        let result: u32 = main_box.roll_dice();
        results.push(result);
    }


    let mut sum: u64 = 0;

    for &result in &results {
        sum += result as u64;
    }

    println!("Median:   {}", median(&results));
    println!("Mean:     {}", sum as f32/iterations);
}
