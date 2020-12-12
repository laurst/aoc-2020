use std::env;
use std::fs;
use std::f32::consts::PI;

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

    let mut world = World {
        ship_x: 0,
        ship_y: 0,
        wp_x: 10,
        wp_y: 1,
    };

    for line in input {
        let (action, nb) = parse_line(&line);
        world.step(action, nb);
    }
    dbg!(world.manhattan_distance());
}

fn parse_line(line: &str) -> (char, i32) {
    let mut it = line.chars();
    let action = it.by_ref().nth(0).unwrap();
    let nb = it.by_ref().collect::<String>().parse::<i32>().unwrap();
    (action, nb)
}

#[derive(Debug)]
struct World {
    ship_x: i32,
    ship_y: i32,
    wp_x: i32,
    wp_y: i32,
}

impl World {
    fn step(&mut self, action: char, nb: i32) {
        match action {
            'N' => self.wp_y += nb,
            'S' => self.wp_y -= nb,
            'E' => self.wp_x += nb,
            'W' => self.wp_x -= nb,
            'L' => self.rotate_wp(nb),
            'R' => self.rotate_wp(-nb),
            'F' => {
                self.ship_x += nb * self.wp_x;
                self.ship_y += nb * self.wp_y;
            }
            _ => panic!(),
        }
    }

    fn rotate_wp(&mut self, angle: i32) {
        let x = self.wp_x as f32;
        let y = self.wp_y as f32;
        let angle_rad = angle as f32 / 180. * PI;
        self.wp_x = (angle_rad.cos() * x) as i32 + ((-angle_rad).sin() * y) as i32;
        self.wp_y = (angle_rad.sin() * x) as i32 + (angle_rad.cos() * y) as i32;
    }

    fn manhattan_distance(&self) -> i32 {
        self.ship_x.abs() + self.ship_y.abs()
    }
}
