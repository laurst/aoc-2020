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

    let input_lines: Vec<&str> = input_data.split_whitespace().collect();

    let width = input_lines[0].len();
    let mut count = 0;

    for (index, line) in input_lines.iter().enumerate() {
        let col = (index * 3) % width;
        if line.chars().nth(col).unwrap() == '#' { count += 1 };
    }
    println!("{}", count);
}
