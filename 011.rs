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

    let mut input: Vec<i32> = input_data
        .split_whitespace()
        .map(|el| str::parse::<i32>(el).unwrap())
        .filter(|&el| el <= 2020)
        .collect();
    input.sort();

    for (index, a) in input.iter().enumerate() {
        if index + 1 == input.len() { break };

        let b = 2020 - a;
        let rest = &input[index..];
        if let Ok(_) = rest.binary_search(&b) {
            println!("a={} b={} a*b={}", a, b, a*b);
        }
    }
}
