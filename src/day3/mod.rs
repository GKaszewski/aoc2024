use std::fs;
use regex::Regex;

fn mul(a: i32, b: i32) -> i32 {
    a * b
}

fn get_all_mul_instructions(input: &str) -> Vec<(i32, i32)> {
    let mut result = Vec::new();
    let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();
    for cap in re.captures_iter(input) {
        let a = cap[1].parse::<i32>().unwrap();
        let b = cap[2].parse::<i32>().unwrap();
        result.push((a, b));
    }

    result
}

fn get_sum_of_all_mul_instructions(input: &str) -> i32 {
    let mul_instructions = get_all_mul_instructions(input);

    let mut result = 0;
    for (a, b) in mul_instructions {
        result += mul(a, b);
    }

    result
}

#[derive(Debug)]
enum Instruction {
    Mul(i32, i32),
    DoNotMul,
    Do
}

fn get_all_instructions(input: &str) -> Vec<Instruction> {
    let mut result = Vec::new();

    let mul_regex = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap(); // regex for mul(a, b)
    let do_not_regex = Regex::new(r"don't\(\)").unwrap(); // regex for don't()
    let do_regex = Regex::new(r"do\(\)").unwrap(); // regex for do()


    // We are going to store the index of the instruction in the input string so later
    // we can use it to put the instructions in the right order
    let mul_instructions_with_indices: Vec<(usize, i32, i32)> = mul_regex.captures_iter(input)
        .map(|cap| {
            let a = cap[1].parse::<i32>().unwrap();
            let b = cap[2].parse::<i32>().unwrap();
            (cap.get(0).unwrap().start(), a, b)
        })
        .collect();

    let do_not_instructions_with_indices: Vec<usize> = do_not_regex.find_iter(input)
        .map(|m| m.start())
        .collect();

    let do_instructions_with_indices: Vec<usize> = do_regex.find_iter(input)
        .map(|m| m.start())
        .collect();

    // We are going to iterate over the input string and check if the current index is in the
    // list of indices of the instructions we found before. If it is, we are going to add the instruction
    // to the result vector

    for (i, _) in input.chars().enumerate() {
        if mul_instructions_with_indices.iter().any(|(index, _, _)| *index == i) {
            let (_, a, b) = mul_instructions_with_indices.iter().find(|(index, _, _)| *index == i).unwrap();
            result.push(Instruction::Mul(*a, *b));
        } else if do_not_instructions_with_indices.iter().any(|index| *index == i) {
            result.push(Instruction::DoNotMul);
        } else if do_instructions_with_indices.iter().any(|index| *index == i) {
            result.push(Instruction::Do);
        }
    }
    

    result
}

fn execute_instructions(instructions: Vec<Instruction>) -> i32 {
    // Simple state machine, we are going to keep track of the result and a boolean that tells us if we should
    // ignore the next mul instruction

    let mut result = 0;
    let mut do_not = false;

    for instruction in instructions {
        match instruction {
            Instruction::Mul(a, b) => {
                if !do_not {
                    result += mul(a, b);
                }
            },
            Instruction::DoNotMul => {
                do_not = true;
            },
            Instruction::Do => {
                do_not = false;
            }
        }
    }

    result
}

pub fn day3() {
    let input = fs::read_to_string("input/day3.txt").unwrap();

    let result = get_sum_of_all_mul_instructions(&input);
    println!("Day 3 part one: {}", result);

    let instructions = get_all_instructions(&input);

    let result = execute_instructions(instructions);
    println!("Day 3 part two: {}", result);
    
}