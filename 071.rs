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

    let lines: Vec<&str> = input_data.split("\n").collect();

    let line_vecs = lines
        .iter()
        .filter(|l| l.len() > 0)
        .map(|l| parse_line(l))
        .collect();

    let mappings = line_vecs_to_map(line_vecs);

    let result = mappings
        .keys()
        .filter(|&k| k != "shiny gold")
        .filter(|k| has_shiny(&mappings, k))
        .count();
    dbg!(result);
}

fn has_shiny(mappings: &HashMap<String, Vec<String>>, key: &str) -> bool {
    if key.to_string() == "shiny gold" { return true };
    let maps = mappings.get(key);
    match maps {
        Some(bags) => {
            for bag in bags {
                if has_shiny(mappings, bag) { return true };
            }
            return false;
        },
        None => return false,
    }
}

fn parse_line(line: &str) -> Vec<(i32, String)> {
    let mut res = vec!();

    let mut line_it = line.split(" bags contain ");

    let first_bag = line_it
        .by_ref()
        .take(1)
        .collect::<String>();
    res.push((1, first_bag));

    for section in line_it
        .collect::<String>()
        .split(", ")
        .filter(|rest| !rest.contains("no other bags"))
    {
        let mut section_it = section.split_whitespace();
        let quantity = section_it
            .by_ref()
            .take(1)
            .collect::<String>()
            .parse::<i32>()
            .unwrap();
        let bag = section_it
            .by_ref()
            .take(2)
            .collect::<Vec<&str>>()
            .join(" ");
        res.push((quantity, bag));
    }
    return res;
}

fn line_vecs_to_map(line_vecs: Vec<Vec<(i32, String)>>) -> HashMap<String, Vec<String>> {
    let mut mappings = HashMap::new();
    for line_vec in line_vecs {
        let mut line_vec_it = line_vec.iter();
        let container = line_vec_it
            .by_ref()
            .take(1)
            .cloned()
            .map(|data| data.1)
            .collect::<String>();
        let contained_bags = line_vec_it
            .by_ref()
            .cloned()
            .map(|entry| entry.1)
            .collect::<Vec<String>>();
        mappings.insert(container, contained_bags);
    }
    mappings
}
