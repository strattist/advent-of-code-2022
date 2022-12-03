use std::fs;
use std::path::Path;

static LOWERCASE_BASE: i32 = 1;
static UPPERCASE_BASE: i32 = 27;

fn get_priority_from_char(c: char) -> i32 {
    if c.is_lowercase() {
        return (c as i32) - ('a' as i32) + LOWERCASE_BASE;
    } else {
        return (c as i32) - ('A' as i32) + UPPERCASE_BASE;
    }
}

fn get_shared_characters(lhs: &str, rhs: &str) -> Vec<char> {
    let copy_lhs = String::from(lhs);
    let mut shared_chars: Vec<char> = Vec::new();
    for c in copy_lhs.chars() {
        if rhs.contains(c) && !shared_chars.contains(&c) {
            shared_chars.push(c)
        }
    }
    shared_chars
}

fn get_priority_for_rucksack(rucksack: &str) -> i32 {
    let mut compartment_0 = rucksack.to_string();
    let compartment_1 = compartment_0.split_off(rucksack.len() / 2);
    let shared_chars = get_shared_characters(&compartment_0, &compartment_1);
    get_priority_from_char(shared_chars[0])
}

fn get_priority_for_group(group: &[&str]) -> i32 {
    let shared_01 = get_shared_characters(group[0], group[1]);
    let shared_01_s: String = shared_01.iter().collect();
    let shared_02 = get_shared_characters(&shared_01_s, group[2]);
    let shared_02_s: String = shared_02.iter().collect();
    let shared_chars = get_shared_characters(&shared_01_s, &shared_02_s);
    get_priority_from_char(shared_chars[0])
}

fn get_sum_of_priorities(input: &str) -> i32 {
    let mut lines = input.lines();
    let mut sum = 0;
    while let Some(line) = lines.next() {
        sum += get_priority_for_rucksack(line);
    }
    sum
}

fn get_sum_of_group_priorities(input: &str) -> i32 {
    let mut sum = 0;
    for three_lines in input.lines().collect::<Vec<_>>().chunks_exact(3) {
        sum += get_priority_for_group(&three_lines);
    }
    sum
}

fn main() {
    let input = include_str!("input.txt");
    let sum = get_sum_of_priorities(input);
    let groupsum = get_sum_of_group_priorities(input);
    println!("smu = {}", sum);
    println!("groupsum = {}", groupsum);
}
