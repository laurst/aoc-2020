use std::env;
use std::fs;

use std::collections::HashSet;

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

    let lines: Vec<&str> = input_data
        .split("\n")
        .filter(|l| l.len() > 0)
        .collect();

    let instructions = lines
        .iter()
        .map(|l| parse_line(l))
        .collect::<Vec<Instruction>>();

    let mut pc = 0;
    let mut acc = 0;
    let mut visited = HashSet::<i32>::new();
    visited.insert(0);

    loop {
        let (pc_inc, acc_inc) = match &instructions[pc as usize] {
            Instruction::Nop => { (1, 0) },
            Instruction::Acc(inc) => { (1, *inc) },
            Instruction::Jmp(inc) => { (*inc, 0) },
        };

        pc += pc_inc;
        if !visited.insert(pc) {
            dbg!(acc);
            break;
        }
        acc += acc_inc;
    }
}

#[derive(Debug)]
enum Instruction {
    Acc(i32),
    Jmp(i32),
    Nop
}

fn parse_line(line: &str) -> Instruction {
    let mut line_iter = line.split_whitespace();
    let instruction = line_iter.next().unwrap();
    let value = line_iter.next().unwrap();
    let inc = value.parse::<i32>().unwrap();

    match instruction {
        "nop" => Instruction::Nop,
        "acc" => Instruction::Acc(inc),
        "jmp" => Instruction::Jmp(inc),
        _ => panic!(),
    }
}
