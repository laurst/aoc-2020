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

    let inputs = [
        (1, 1),
        (3, 1),
        (5, 1),
        (7, 1),
        (1, 2),
    ];
    let mut counts = [0; 5];

    for (index, line) in input_lines.iter().enumerate() {
        for (counter_index, (x_step, y_step)) in inputs.iter().enumerate() {
            if index % y_step == 1 { continue };
            let col = (index * x_step / y_step) % width;
            if line.chars().nth(col).unwrap() == '#' { counts[counter_index] += 1 };
        }
    }
    let result: u64 = counts.iter().fold(1, |a, b| a*b);
    println!("{:?}", counts);
    println!("{}", result);
}
