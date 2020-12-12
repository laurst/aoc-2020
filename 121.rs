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

    let input: Vec<&str> = input_data
        .split_whitespace()
        .collect();

    let mut ship = Ship { dir: 90, x: 0, y: 0 };

    for line in input {
        let (action, nb) = parse_line(&line);
        ship.step(action, nb);
    }
    dbg!(ship.manhattan_distance());
}

fn parse_line(line: &str) -> (char, i32) {
    let mut it = line.chars();
    let action = it.by_ref().nth(0).unwrap();
    let nb = it.by_ref().collect::<String>().parse::<i32>().unwrap();
    (action, nb)
}

#[derive(Debug)]
struct Ship {
    dir: i32,
    x: i32,
    y: i32
}

impl Ship {
    fn step(&mut self, action: char, nb: i32) {
        match action {
            'N' => self.y += nb,
            'S' => self.y -= nb,
            'E' => self.x += nb,
            'W' => self.x -= nb,
            'L' => self.dir = (self.dir + 360 - nb) % 360,
            'R' => self.dir = (self.dir + nb) % 360,
            'F' => match self.dir {
                0 => self.y += nb,
                180 => self.y -= nb,
                90 => self.x += nb,
                270 => self.x -= nb,
                _ => panic!(),
            },
            _ => panic!(),
        }
    }
    fn manhattan_distance(&self) -> i32 {
        self.x.abs() + self.y.abs()
    }
}
