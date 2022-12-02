use std::fs;
use std::path::Path;

// Rock = 0
// Paper = 1
// Scissor = 2

fn get_index(c: &str) -> i32 {
    let index = match c {
        "A" => 0,
        "B" => 1,
        "C" => 2,
        "X" => 0,
        "Y" => 1,
        "Z" => 2,
        &_ => -1,
    };
    index
}

fn get_index_for_end(p2: &str, p1: i32) -> i32 {
    let shape = match p2 {
        "X" => (p1 - 1 + 3) % 3,
        "Y" => p1,
        "Z" => (p1 + 1) % 3,
        &_ => -1,
    };
    let word = match p2 {
        "X" => "lose", 
        "Y" => "draw", 
        "Z" => "win",
        &_ => ""
    };
    shape
}

fn get_score_from_line(line: String) -> i32 {
    let v: Vec<&str> = line.split(' ').collect();
    let p1 = get_index(v[0]);
    let p2 = get_index(v[1]);
    let mut score = p2 + 1;
    if p1 == p2 {
        score += 3;
    } else if (p1 + 1) % 3 == p2 {
        score += 6;
    }
    score
}

fn get_tricked_score_from_line(line: String) -> i32 {
    let v: Vec<&str> = line.split(' ').collect();
    let p1 = get_index(v[0]);
    let p2 = get_index_for_end(v[1], p1);
    let mut score = p2 + 1;
    if p1 == p2 {
        score += 3;
    } else if (p1 + 1) % 3 == p2 {
        score += 6;
    }
    score
}

fn read_input() -> String {
    let input_dir = Path::new(file!()).parent().unwrap();
    let input_file = input_dir.join("input.txt");
    let content = fs::read_to_string(input_file).unwrap();
    content
}

fn get_strategy_guide(input: String) -> i32 {
    let mut lines = input.lines();
    let mut sum = 0;
    while let Some(line) = lines.next() {
        sum += get_score_from_line(line.to_string());
    }
    sum
}

fn get_tricked_strategy_guide(input: String) -> i32 {
    let mut lines = input.lines();
    let mut sum = 0;
    while let Some(line) = lines.next() {
        sum += get_tricked_score_from_line(line.to_string());
    }
    sum
}

fn main() {
    let input = read_input();
    let _sum_part1 = get_strategy_guide("".to_string());
    let sum_part2 = get_tricked_strategy_guide(input);
    println!("sum = {}", sum_part2)
}
