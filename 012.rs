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

    for (first_index, a) in input.iter().enumerate() {
        if first_index + 2 >= input.len() { break };

        let b_rest = &input[first_index..];

        for (sec_index, b) in b_rest.iter().enumerate() {
            if sec_index + 1 >= input.len() { break };
            if a + b >= 2020 { break };

            let c_rest = &input[sec_index..];

            let c = 2020 - a - b;

            if let Ok(_) = c_rest.binary_search(&c) {
                println!("a={} b={} c={} a*b*c={}", a, b, c, a*b*c);
            }
        }
    }
}
