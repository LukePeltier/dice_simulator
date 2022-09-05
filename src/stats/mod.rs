pub fn median(data: &Vec<u32>) -> u32 {
    let mut sorted: Vec<u32> = data.clone();

    sorted.sort();

    let middle = (((sorted.len() - 1) as f32) / 2.0).round() as usize;

    match sorted.get(middle) {
        Some(x) => *x,
        None => 0,
    }
}
