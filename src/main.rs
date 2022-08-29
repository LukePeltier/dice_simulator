use rand::prelude::*;
use std::env;

fn median(data: &Vec<u32>) -> u32 {
    let mut sorted: Vec<u32> = data.clone();

    sorted.sort();

    let middle = (((sorted.len()-1) as f32)/2.0).round() as usize;

    match sorted.get(middle) {
        Some(x) => *x,
        None => 0
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let mut rng: ThreadRng = rand::thread_rng();
    let mut sum: u32 = 0;
    let mut iterations: f32 = 1000.0;
    if args.len() > 1 {
        iterations= match args[1].parse::<f32>() {
            Ok(res) => {
                res
            },
            Err(_err) => {
                println!("Could not find number, using default");
                iterations
            }
        };
    }
    println!("Using  iterations: {}", iterations);

    let mut results: Vec<u32> = Vec::new();

    for _ in 0..iterations as u32{
        let x: u32 = rng.gen_range(1..=20);
        sum += x;
        results.push(x);
    }
    println!("Median:   {}", median(&results));
    println!("Mean:     {}", sum as f32/iterations);
}
