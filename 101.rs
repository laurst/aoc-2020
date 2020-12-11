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

    let mut data = vec!(0);
    data.extend(&input);
    data.sort();
    data.push(data.last().unwrap() + 3);


    let mut diff_1 = 0;
    let mut diff_3 = 0;
    for slice in data.windows(2) {
        let diff = slice[1] - slice[0];
        if diff == 1 { diff_1 += 1 };
        if diff == 3 { diff_3 += 1 };
    }
    dbg!(diff_1, diff_3, diff_1 * diff_3);
}
