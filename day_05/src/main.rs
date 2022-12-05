use std::fmt;

fn main() {
    let input = include_str!("input.txt");
    let moved_supplies = get_moved_supplies(input, false);
    println!("moved_supplies = {}", moved_supplies);
    let moved_supplies = get_moved_supplies(input, true);
    println!("moved_supplies = {}", moved_supplies);
}

struct Instruction {
    n: usize,
    source: usize,
    dest: usize,
}

impl fmt::Display for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "move {} from {} to {}", self.n, self.source, self.dest)
    }
}

impl Instruction {
    fn process(&self, stacks: Vec<Vec<char>>, with_cratemover_9001: bool) -> Vec<Vec<char>> {
        let mut processed_stacks = Vec::from(stacks);
        let length = processed_stacks[self.source].len().saturating_sub(self.n);
        let mut extracted = processed_stacks[self.source].split_off(length);
        if !with_cratemover_9001 {
            extracted.reverse();
        }
        processed_stacks[self.dest].extend(&extracted);
        processed_stacks
    }
}

fn get_moved_supplies(input: &str, with_cratemover_9001: bool) -> String {
    let (crates, instructions) = input.split_once("\n\n").unwrap();
    let mut stacks: Vec<Vec<char>> = initialize_crates(crates);
    let instructions: Vec<Instruction> = parse_instructions(instructions);
    for i in instructions {
        stacks = i.process(stacks, with_cratemover_9001);
    }
    stacks
        .iter()
        .filter_map(|stack| stack.last())
        .collect::<String>()
}

fn initialize_crates(input: &str) -> Vec<Vec<char>> {
    let mut stacks = Vec::<Vec<char>>::with_capacity(9);

    input.lines().rev().skip(1).for_each(|line| {
        line.chars()
            .skip(1)
            .step_by(4)
            .enumerate()
            .filter(|(_, item)| !item.is_ascii_whitespace())
            .for_each(|(i, item)| {
                if i >= stacks.len() {
                    let mut new_vec : Vec<char> = Vec::new();
                    new_vec.push(item);
                    stacks.push(new_vec);
                } else {
                    stacks[i].push(item);
                }
            })
    });
    stacks
}

fn parse_instruction(input: &str) -> Instruction {
    let words : Vec<&str> = input.split(' ').collect();
    let mut integers : Vec<usize> = Vec::new();
    for word in words {
        match word.parse::<usize>() {
            Ok(n) => integers.push(n),
            Err(_) => continue,
        }
    }
    let instruction = Instruction {
        n: integers[0],
        source: integers[1] - 1,
        dest: integers[2] - 1,
    };
    instruction
}

fn parse_instructions(input: &str) -> Vec<Instruction> {
    let instructions: Vec<Instruction> = input
        .lines()
        .map(|line| parse_instruction(line))
        .collect();
    instructions
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = "    [D]    
[N] [C]    
[Z] [M] [P]
1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2";

    #[test]
    fn test_initialize_crates() {
        let (crates, _) = EXAMPLE_INPUT.split_once("\n\n").unwrap();
        let stacks: Vec<Vec<char>> = initialize_crates(crates);
        assert_eq!(stacks.len(), 3);
        assert_eq!(stacks[1].len(), 3);
        assert_eq!(stacks[2][0], 'P');
    }

    #[test]
    fn test_parse_instruction() {
        let instructionstr = "move 1 from 2 to 1";
        let instruction : Instruction = parse_instruction(instructionstr);
        assert_eq!(instruction.n, 1);
        assert_eq!(instruction.source, 1);
        assert_eq!(instruction.dest, 0);
    }

    #[test]
    fn test_parse_instructions() {
        let (_, instructions) = EXAMPLE_INPUT.split_once("\n\n").unwrap();
        let instructions: Vec<Instruction> = parse_instructions(instructions);
        assert_eq!(instructions.len(), 4);
        assert_eq!(instructions[1].n, 3);
        assert_eq!(instructions[1].source, 0);
        assert_eq!(instructions[1].dest, 2);
    }
}