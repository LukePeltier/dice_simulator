mod dice_box;
mod stats;
use crate::dice_box::DiceBox;
use crate::stats::median;

use core::panic;
use std::collections::BTreeSet;
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
                Ok(res) => res,
                Err(_err) => {
                    println!("Could not find number, using default");
                    iterations
                }
            };
            println!("Received dice string: {}", dice_string);
            println!("Using  iterations: {}", iterations);
        }
        _ => {
            usage();
            return;
        }
    }
    let main_box = DiceBox::from_string(dice_string);
    let mut results: Vec<u32> = vec![];

    let start_time = SystemTime::now();

    for _ in 0..iterations as u32 {
        let result: u32 = main_box.roll_dice();
        results.push(result);
    }

    let mut sum: u64 = 0;

    for &result in &results {
        sum += result as u64;
    }

    let median = median(&results);
    let mean = sum as f32 / iterations;

    let true_mean = main_box.get_mean();

    let lowest_possible = main_box.get_lowest();
    let highest_possible = main_box.get_highest();

    let all_outcomes = main_box.get_all_outcomes();

    let mut missing_outcomes: BTreeSet<u32> = BTreeSet::new();

    for i in lowest_possible..=highest_possible {
        if !all_outcomes.contains(&i) {
            missing_outcomes.insert(i);
        }
    }

    let end_time = SystemTime::now();

    let time_diff = match end_time.duration_since(start_time) {
        Ok(duration) => duration,
        Err(err) => panic!("Unable to compute time difference since start, {}", err),
    };

    println!("");
    println!("----------RESULTS----------");
    println!("Median:           {}", median);
    println!("Mean:             {}", mean);
    println!("---------------------------");
    println!("True Mean:        {}", true_mean);
    println!("Min:              {}", lowest_possible);
    println!("Max:              {}", highest_possible);
    println!("");
    println!("Num of Outcomes:  {}", all_outcomes.len());

    // println!("All Outcomes:     {}", all_outcomes);
    let total_seconds = time_diff.as_secs();
    let hours = total_seconds / 3600;
    let minutes_seconds = total_seconds % 3600;
    let minutes = minutes_seconds / 60;
    let seconds = minutes_seconds % 60;
    println!("Time to calculate: {}h {}m {}s ", hours, minutes, seconds);
}
