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

    let result = input_data
        .split('\n')
        .filter(|line| line.len() > 0)
        .filter(|line| is_password_valid(&line))
        .count();

    println!("{:?}", result);
}

fn is_password_valid(line: &str) -> bool {
    let mut iterator = line.chars().fuse();

    let lower = iterator
        .by_ref()
        .take_while(|&ch| ch != '-')
        .collect::<String>()
        .parse::<usize>().unwrap();

    let upper = iterator
        .by_ref()
        .take_while(|&ch| ch != ' ')
        .collect::<String>()
        .parse::<usize>().unwrap();

    let letter = iterator.nth(0).unwrap();
    let _ = iterator.nth(1);

    let rest: String = iterator.collect();

    let count = rest.chars()
        .filter(|&ch| ch == letter)
        .count();

    if count < lower || count > upper {
        return false
    } else {
        return true
    }
}
