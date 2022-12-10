use std::str::FromStr;

fn main() {
    let input = include_str!("input.txt");
    let to_inspect = vec![20, 60, 100, 140, 180, 220];
    let sum = get_sum_of_signal_strengths(input, &to_inspect);
    println!("sum = {}", sum);
    get_capital_letters(input);
}

#[derive(Debug, PartialEq)]
enum Instruction {
    Noop,
    Addx(i32),
}

impl FromStr for Instruction {
    type Err = ();
    fn from_str(input: &str) -> Result<Instruction, Self::Err> {
        let instruction: Vec<&str> = input.split(' ').collect();
        match instruction[0] {
            "noop" => Ok(Instruction::Noop),
            "addx" => Ok(Instruction::Addx(instruction[1].parse::<i32>().unwrap())),
            _   => Err(()),
        }
    }
} 

fn cycle(register: i32, nticks: &mut u32, nticks_target: Option<&u32>) -> Option<i32> {
    *nticks += 1;
    match nticks_target {
        Some(target) => {
            if *nticks == *target {
                let x = *nticks as i32 * register;
                Some(x)
            } else {
                None
            }
        }
        None => None
    }
}

fn process_instruction(instruction: Instruction, register: &mut i32, nticks: &mut u32, nticks_target: Option<&u32>) -> Option<i32> {
    match instruction {
        Instruction::Noop => {
            cycle(*register, nticks, nticks_target)
        },
        Instruction::Addx(n) => {
            let before = cycle(*register, nticks, nticks_target);
            let after = cycle(*register, nticks, nticks_target);
            *register += n;
            match (before, after) {
                (Some(_), None) => before,
                (None, Some(_)) => after,
                (_, _) => None
            }
        }
    }
}

fn draw_cycle(register: i32, nticks: &mut u32, console: &mut String) {
    *nticks += 1;
    // -1 since nticks starts at 1
    let c = (*nticks % 40) as i32 - 1;
    let sprite = (register - 1).max(0)..=(register + 1).min(39);
    if sprite.contains(&c) {
        console.push_str("#");
    } else if c == -1 {
        console.push_str("\n");
    } else {
        console.push_str(".");
    }
}


fn draw_instruction(instruction: Instruction, register: &mut i32, nticks: &mut u32, console: &mut String) {
    match instruction {
        Instruction::Noop => {
            draw_cycle(*register, nticks, console)
        },
        Instruction::Addx(n) => {
            draw_cycle(*register, nticks, console);
            draw_cycle(*register, nticks, console);
            *register += n;
        }
    }
}

fn get_signal_states(input: &str, to_inspect: &Vec<u32>) -> Vec<i32> {
    let lines: Vec<&str> = input.lines().collect();
    let mut states: Vec<i32> = Vec::new();
    let mut register: i32 = 1;
    let mut nticks: u32 = 0;
    let mut next_inspection: usize = 0;
    for line in lines {
        let instruction = Instruction::from_str(line).unwrap();
        let state = process_instruction(
            instruction, &mut register, &mut nticks, to_inspect.get(next_inspection));
        match state {
            Some(x) => {
                states.push(x);
                next_inspection += 1;
            },
            _ => (),
        }
    }
    states
}

fn get_sum_of_signal_strengths(input: &str, to_inspect: &Vec<u32>) -> i32 {
    let states = get_signal_states(input, &to_inspect);
    states.iter().sum()
}

fn get_capital_letters(input: &str) {
    let mut console = String::from("");
    let lines: Vec<&str> = input.lines().collect();
    let mut register: i32 = 1;
    let mut nticks: u32 = 0;
    for line in lines {
        let instruction = Instruction::from_str(line).unwrap();
        draw_instruction(instruction, &mut register, &mut nticks, &mut console);
    }

    println!("{}", console);
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = include_str!("example_input.txt");

    #[test]
    fn test_instruction_from_str() {
        let testset: Vec<&str> = EXAMPLE_INPUT.lines().take(10).collect();
        let expected: Vec<Instruction> = vec![
            Instruction::Addx(15),
            Instruction::Addx(-11),
            Instruction::Addx(6),
            Instruction::Addx(-3),
            Instruction::Addx(5),
            Instruction::Addx(-1),
            Instruction::Addx(-8),
            Instruction::Addx(13),
            Instruction::Addx(4),
            Instruction::Noop];
        for (i, line) in testset.iter().enumerate() {
            let instruction = Instruction::from_str(line).unwrap();
            assert_eq!(instruction, expected[i]);
        }
    }

    #[test]
    fn test_instruction_process() {
        let testset: Vec<&str> = EXAMPLE_INPUT.lines().take(10).collect();
        let expected_nticks: Vec<u32> = vec![
            2,
            2 + 2,
            2 + 2 + 2,
            2 + 2 + 2 + 2,
            2 + 2 + 2 + 2 + 2,
            2 + 2 + 2 + 2 + 2 + 2,
            2 + 2 + 2 + 2 + 2 + 2 + 2,
            2 + 2 + 2 + 2 + 2 + 2 + 2 + 2,
            2 + 2 + 2 + 2 + 2 + 2 + 2 + 2 + 2,
            2 + 2 + 2 + 2 + 2 + 2 + 2 + 2 + 2 + 1
        ];
        let expected_register: Vec<i32> = vec![
            1 + 15, 
            1 + 15 - 11,
            1 + 15 - 11 + 6,
            1 + 15 - 11 + 6 - 3,
            1 + 15 - 11 + 6 - 3 + 5,
            1 + 15 - 11 + 6 - 3 + 5 - 1,
            1 + 15 - 11 + 6 - 3 + 5 - 1 - 8,
            1 + 15 - 11 + 6 - 3 + 5 - 1 - 8 + 13,
            1 + 15 - 11 + 6 - 3 + 5 - 1 - 8 + 13 + 4,
            1 + 15 - 11 + 6 - 3 + 5 - 1 - 8 + 13 + 4];
        let mut register = 1;
        let mut nticks = 0;
        for (i, line) in testset.iter().enumerate() {
            let instruction = Instruction::from_str(line).unwrap();
            let _ = process_instruction(instruction, &mut register, &mut nticks, None);
            assert_eq!(nticks, expected_nticks[i]);
            assert_eq!(register, expected_register[i]);
        }
    }

    #[test]
    fn test_get_signal_states() {
        let to_inspect: Vec<u32> = vec![20, 60, 100, 140, 180, 220];
        let expected_states: Vec<i32> = vec![420, 1140, 1800, 2940, 2880, 3960]; 
        let states = get_signal_states(EXAMPLE_INPUT, &to_inspect);
        assert_eq!(states.len(), to_inspect.len());
        for (i, state) in states.iter().enumerate() {
            assert_eq!(*state, expected_states[i]);
        }
    }

    #[test]
    fn test_get_sum_of_signal_strengths() {
        let to_inspect: Vec<u32> = vec![20, 60, 100, 140, 180, 220];
        let sum = get_sum_of_signal_strengths(EXAMPLE_INPUT, &to_inspect);
        assert_eq!(sum, 13140);
    }
}
