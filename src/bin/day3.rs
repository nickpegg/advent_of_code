use regex::Regex;
use std::error::Error;

#[derive(Debug)]
enum Instruction {
    Mul { x: i32, y: i32 },
    Do,
    Dont,
}

fn main() {
    part1();
    part2();
}

fn part1() {
    let result = parse_input()
        .unwrap()
        .into_iter()
        .fold(0, |acc, i| acc + compute(i));
    println!("Part 1 result: {}", result);
}

fn part2() {
    let mut result = 0;
    let mut operate = true;
    for instr in parse_input().unwrap() {
        match instr {
            Instruction::Mul {x, y} => {
                if operate {
                    result += x * y;
                }
            },
            Instruction::Do => { operate = true; },
            Instruction::Dont => { operate = false; },
        }
    }
    println!("Part 2 result: {}", result);
}

fn compute(instr: Instruction) -> i32 {
    match instr {
        Instruction::Mul {x, y} => x * y,
        Instruction::Do | Instruction::Dont => 0,
    }
}

fn parse_input() -> Result<Vec<Instruction>, Box<dyn Error>> {
    let mut instructions = Vec::new();
    let input = include_str!("../../data/day3.txt");

    let re = Regex::new(r"(?<instr>mul|do|don't)\((?<args>[\d,]+)?\)").unwrap();
    for capture in re.captures_iter(input) {
        let found = match &capture["instr"] {
            "mul" => {
                let mut args = capture["args"].split(",");
                Instruction::Mul {
                    x: args.next().unwrap().parse()?,
                    y: args.next().unwrap().parse()?,
                }
            },
            "do" => Instruction::Do,
            "don't" => Instruction::Dont,
            i => return Err(format!("Unrecognized instruction: {}", i).into()),
        };
        instructions.push(found);
    }

    Ok(instructions)
}
