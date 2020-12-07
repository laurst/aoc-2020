use std::env;
use std::fs;

use std::collections::HashSet;

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

    let group_data: Vec<&str> = input_data.split("\n\n").collect();

    let sets: Vec<HashSet<char>> = group_data
        .iter()
        .map(|s| s.chars().filter(|&c| c != '\n').collect::<HashSet<char>>())
        .collect();

    let sum: usize = sets
        .iter()
        .map(|s| s.len())
        .sum();
    println!("{}", sum);
}
