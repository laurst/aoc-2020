use std::env;
use std::fs;

use std::collections::{HashSet, HashMap};

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

    let required_fields: HashSet<String> =
        ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"]
        .iter()
        .map(|s| s.to_string())
        .collect();

    let passport_strings: Vec<&str> = input_data.split("\n\n").collect();

    let mut count = 0;
    for passport_string in passport_strings.iter() {
        let data: HashMap<String, String> = passport_string
            .split_whitespace()
            .map(|field| field.split(':').collect::<Vec<&str>>())
            .map(|vec| (vec[0].to_string(), vec[1].to_string()))
            .collect();

        let keys: HashSet<String> = data.keys().cloned().collect();
        if keys.is_superset(&required_fields) { count += 1 };
    }
    println!("{}", count);
}
