mod dice_box;
mod stats;
use crate::dice_box::DiceBox;
use crate::dice_box::die::Die;
use crate::stats::median;

use core::panic;
use std::env;
use std::time::SystemTime;

fn usage() {
    println!("Usage: dice_simulator [dice string] [iterations]");
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
    let main_box = DiceBox::new(vec![Die::new(1, 20, 0, 0)], 0);
    let mut results: Vec<u32> = vec![];

    let start_time = SystemTime::now();

    for _ in 0..iterations as u32{
        let result: u32 = main_box.roll_dice();
        results.push(result);
    }

    let mut sum: u64 = 0;

    for &result in &results {
        sum += result as u64;
    }

    let median = median(&results);
    let mean = sum as f32/iterations;
    let end_time = SystemTime::now();

    let time_diff = match end_time.duration_since(start_time){
        Ok(duration) => duration,
        Err(err) => panic!("Unable to compute time difference since start, {}", err)
    };

    println!("");
    println!("----------RESULTS----------");
    println!("Median:   {}", median);
    println!("Mean:     {}", mean);
    println!("");
    let total_seconds = time_diff.as_secs();
    let hours = total_seconds / 3600;
    let minutes_seconds = total_seconds % 3600;
    let minutes = minutes_seconds / 60;
    let seconds = minutes_seconds % 60;
    println!("Time to calculate: {}h {}m {}s ", hours, minutes, seconds);
}
