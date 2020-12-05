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

        if keys.is_superset(&required_fields) && valid(&data) {
            count += 1
        };

    }
    println!("{}", count);
}

fn valid(map: &HashMap<String, String>) -> bool {
    let byr = map.get("byr").unwrap().parse::<i32>().unwrap();
    if byr < 1920 || byr > 2002 { return false };

    let iyr = map.get("iyr").unwrap().parse::<i32>().unwrap();
    if iyr < 2010 || iyr > 2020 { return false };

    let eyr = map.get("eyr").unwrap().parse::<i32>().unwrap();
    if eyr < 2020 || eyr > 2030 { return false };

    let hgt = map.get("hgt").unwrap();
    if hgt.ends_with("cm") {
        let value = hgt.trim_end_matches("cm").parse::<i32>().unwrap();
        if value < 150 || value > 193 { return false };
    } else if hgt.ends_with("in") {
        let value = hgt.trim_end_matches("in").parse::<i32>().unwrap();
        if value < 59 || value > 76 { return false };
    } else { return false };

    let hcl = map.get("hcl").unwrap();
    let mut it = hcl.chars();
    if it.by_ref().nth(0).unwrap() != '#' { return false };
    for (idx, chr) in it.enumerate() {
        if !chr.is_ascii_hexdigit() { return false };
        if idx > 5 { return false };
    }

    let ecl = map.get("ecl").unwrap();
    let valid_ecl: HashSet<String> =
        ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"]
        .iter()
        .map(|s| s.to_string())
        .collect();
    if !valid_ecl.contains(ecl) { return false };

    let pid = map.get("pid").unwrap();
    if pid.len() != 9 { return false };
    if !pid.chars().all(|chr| chr.is_ascii_digit()) { return false };

    true
}
