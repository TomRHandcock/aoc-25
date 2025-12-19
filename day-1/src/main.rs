use std::{fmt::Display, fs};

enum Direction {
    Left { distance: i16 },
    Right { distance: i16 }
}

impl Display for Direction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Direction::Left { distance } => write!(f,"Left(distance={distance})"),
            Direction::Right { distance } => write!(f,"Right(distance={distance})")
        }
    }
}

fn load_file_lines(path: &str) -> Vec<String> {
    let contents = fs::read_to_string(path).expect("Failed to open file");
    let lines: Vec<String> = contents.split("\n").map(|line| line.to_string()).collect();
    lines.iter().take(lines.len()-1).map(|line| line.to_string()).collect::<Vec<String>>()
}

fn process_instruction(instruction: &Direction, counter: &mut i16, passes_zero: &mut i16) {
    match instruction {
        Direction::Left { distance } => {
            for _n in 0..*distance {
                *counter -= 1;
                if *counter == -1 {
                    *counter = 99;
                }
                if *counter == 0 {
                    *passes_zero += 1;
                }
            }
        },
        Direction::Right { distance } => {
            for _n in 0..*distance {
                *counter += 1;
                if *counter == 100 {
                    *counter = 0;
                }
                if *counter == 0 {
                    *passes_zero += 1;
                }
            }
        }
    }
}

fn passes_zero_count(instruction: &Direction, counter: &i16) -> i16 {
    match instruction {
        Direction::Right { distance } => ((counter + distance).div_euclid(100)),
        Direction::Left { distance } => {
            let new_raw_pos = counter - distance;
            let rotations = ((new_raw_pos).div_euclid(-100)).abs();
            let was_on_zero = *counter == 0;
            let now_on_zero = new_raw_pos % -100 == 0;
            match rotations {
                _ if was_on_zero && !now_on_zero => rotations - 1,
                _ if now_on_zero && !was_on_zero => rotations + 1,
                _ => rotations,
            }
        },
    }
}

fn process_instruction_array(instructions: Vec<Direction>, counter: &mut i16) -> i16 {
    let mut times_hit_zero : i16= 0;
    for instruction in instructions {
        process_instruction(&instruction, counter, &mut times_hit_zero);
        println!("Processed {instruction}, Safe dial is at {counter}, pass zero count at {times_hit_zero}");
    }
    times_hit_zero
}

fn main() {
    let contents = load_file_lines("./data.txt");
    let instructions = contents.iter().map(|line| {
        let chars = line.chars();
        let direction_char: char = chars.collect::<Vec<char>>()[0];
        let distance_chars = line.chars().filter(|character| character.is_numeric()).collect::<Vec<char>>();
        let distance = distance_chars.iter().collect::<String>();
        
        match direction_char {
            'L' => Direction::Left { distance: distance.parse::<i16>().expect("Failed to parse i8") },
            'R' => Direction::Right { distance: distance.parse::<i16>().expect("failed to parse i8") },
            _ => panic!("Invalid direction char")
        }
    }).collect::<Vec<Direction>>();

    let mut safe_dial: i16 = 50;
    let times_hit_zero = process_instruction_array(instructions, &mut safe_dial);
    println!("Code is {times_hit_zero}");
}
