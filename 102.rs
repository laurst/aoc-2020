use std::env;
use std::fs;

use std::collections::HashMap;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = args[1].clone();
    let input_data = match fs::read_to_string(&filename) {
        Ok(data) => data,
        Err(_) => {
            let filename = filename.replace("2.txt", "1.txt");
            fs::read_to_string(&filename).unwrap()
        }
    };

    let input: Vec<i64> = input_data
        .split_whitespace()
        .map(|el| str::parse::<i64>(el).unwrap())
        .collect();

    let mut data = vec!(0);
    data.extend(&input);
    data.sort();
    data.push(data.last().unwrap() + 3);

    let differences: Vec<i64> = data
        .windows(2)
        .map(|slice| slice[1] - slice[0])
        .collect();

    let consecutive_ones: Vec<usize> = differences
        .split(|&num| num == 3)
        .map(|slice| slice.len())
        .filter(|&nb| nb>1)
        .collect();

    let mut consecutives_counts: HashMap<i64, u32> = HashMap::new();
    for nb in &consecutive_ones {
        let count = consecutives_counts.entry(*nb as i64).or_insert(0);
        *count += 1;
    }

    let result = consecutives_counts
        .iter()
        .map(|(k, v)| (1 + k * (k-1)/2).pow(*v))
        .fold(1, |a, b| a*b);
    dbg!(result);
}
