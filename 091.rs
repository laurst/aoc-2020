use std::env;
use std::fs;

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

    let target = find_target(&input).unwrap();
    dbg!(target);
}

fn find_target(input_data: &Vec<i64>) -> Option<i64> {
    let window_size = 25;

    for (index, nb) in input_data.iter().enumerate().skip(window_size) {
        if !is_ok(&input_data[index-window_size..index], *nb) {
            return Some(*nb);
        }
    }
    None
}

fn is_ok(window: &[i64], nb: i64) -> bool {
    for (index, a) in window.iter().enumerate() {
        if index + 1 == window.len() { return false };

        let b = nb - a;
        let rest = &window[index..];
        if rest.contains(&b) {
            return true;
        }
    }
    false
}
