use std::fs;
use std::path::Path;

fn read_input() -> String {
    let input_dir = Path::new(file!()).parent().unwrap();
    let input_file = input_dir.join("input.txt");
    let content = fs::read_to_string(input_file).unwrap();
    content
}

// fn get_maximum_calories(input: String) -> i32 {
//     let mut lines = input.lines();
//     let mut maximum: i32 = 0;
//     let mut current: i32 = 0;
//     while let Some(line) = lines.next() {
//         match line.parse::<i32>() {
//             Ok(value) => current += value,
//             Err(_) => {
//                 maximum = if current > maximum { current } else { maximum };
//                 current = 0
//             }
//         }
//     }
//     maximum
// }

fn get_sum_of_top_three_calories(input: String) -> i32 {
    let mut lines = input.lines();
    let mut calories_per_elf = Vec::<i32>::new();
    let mut current: i32 = 0;
    while let Some(line) = lines.next() {
        match line.parse::<i32>() {
            Ok(value) => current += value,
            Err(_) => {
                calories_per_elf.push(current);
                current = 0;
            }
        }
    }
    calories_per_elf.sort_by(|a, b| b.cmp(a));
    let sum = calories_per_elf[0] + calories_per_elf[1] + calories_per_elf[2];
    sum
}

fn main() {
    let input = read_input();
    // let maximum = get_maximum_calories(input);
    let sum = get_sum_of_top_three_calories(input);
    println!("sum = {}", sum)
}
