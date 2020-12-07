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
    let existing_ids = input_data
        .split_whitespace()
        .map(parse_seat)
        .map(calculate_id)
        .collect::<HashSet<u32>>();

    let min = existing_ids.iter().min().unwrap();
    let max = existing_ids.iter().max().unwrap();
    let all_ids = (*min..*max+1).collect::<HashSet<u32>>();

    println!("{:?}", all_ids.difference(&existing_ids));
}

fn parse_seat(seat: &str) -> (u32, u32) {
    let mut row = 0;
    let mut col = 0;

    for chr in seat.chars() {
        if chr == 'B' {
            row <<= 1;
            row |= 1;
        } else if chr == 'F' {
            row <<= 1;
        } else if chr == 'L' {
            col <<= 1;
        } else if chr == 'R' {
            col <<= 1;
            col |= 1;
        }
    }
    (row, col)
}

fn calculate_id((row, col): (u32, u32)) -> u32 {
    row * 8 + col
}
