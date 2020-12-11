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

    let mut layout: Vec<String> = input_data
        .split_whitespace()
        .map(|line| line.to_string())
        .collect();

    loop {
        let (changed, new_layout) = step(&layout);
        if !changed { break };
        layout = new_layout;
    }
    dbg!(count_busy(&layout));
}

fn step(layout: &Vec<String>) -> (bool, Vec<String>) {
    let width = layout[0].len();
    let length = layout.len();

    let mut result = Vec::new();
    for y in 0..length {
        let mut row = String::new();
        for x in 0..width {
            row.push(next_position(&layout, x, y));
        }
        result.push(row);
    }
    (&result != layout, result)
}

fn next_position(layout: &Vec<String>, row: usize, col: usize) -> char {
    let this = get_seat(&layout, row, col);
    let neighbors = get_neighbors(&layout, row as i32, col as i32);
    if this == 'L' && !neighbors.contains(&'#') { return '#' };
    if this == '#'
        && neighbors.iter().filter(|&c| c == &'#').count() >= 5 { return 'L' };
    return this;
}

fn get_neighbors(layout: &Vec<String>, x: i32, y: i32) -> Vec<char> {
    let width = layout[0].len() as i32;
    let length = layout.len() as i32;
    let mut result = Vec::new();
    for di in -1..=1 {
        for dj in -1..=1 {
            if di == 0 && dj == 0 { continue };
            for factor in 1..length {
                let i = di * factor;
                let j = dj * factor;
                if x+j < 0 || y+i < 0 || x+j >= width || y+i >= length { continue };
                let next_seat = get_seat(&layout, (x+j) as usize, (y+i) as usize);
                if next_seat == '.' { continue };
                result.push(next_seat);
                break;
            }
        }
    }
    result
}

fn get_seat(layout: &Vec<String>, x: usize, y: usize) -> char {
    let row = &layout[y];
    row.chars().nth(x).unwrap()
}

fn count_busy(layout: &Vec<String>) -> usize {
    layout.iter()
        .map(|l| l.chars().filter(|&c| c == '#').count())
        .sum()
}
