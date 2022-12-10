use std::collections::HashMap;
use std::fmt;

fn main() {
    let input = include_str!("input.txt");
    let dir = parse_filesystem(input);
    let sum_of_low_sized_dirs = get_sum_of_low_sized_dirs(&dir);
    println!("sum_of_low_sized_dirs = {}", sum_of_low_sized_dirs);
    let minimum_sized_dir = get_minimum_sized_dir_to_delete(&dir);
    println!("minimum_sized_dir = {}", minimum_sized_dir);
}



#[derive(Debug, PartialEq, Eq)]
enum Command<'a> {
    CD(&'a str),
    LS,
}

#[derive(Debug)]
struct File {
    name: String,
    size: usize,
}

impl fmt::Display for File {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result { 
        write!(f, "{} ({})", self.name, self.size)
    }
}

fn get_sum_of_low_sized_dirs(dir: &HashMap<String, usize>) -> usize {
    dir.values().filter(|s| **s <= 100000).sum()
}

fn get_minimum_sized_dir_to_delete(dir: &HashMap<String, usize>) -> usize {
    let mut sizes = dir.values().copied().collect::<Vec<usize>>();

    let total = dir[""];

    sizes.sort();

    sizes
        .iter()
        .copied()
        .find(|s| 70000000 - (total - *s) >= 30000000)
        .unwrap()
}

fn parse_command_line<'a>(input: &'a str) -> Command<'a> {
    let mut args = input.split_ascii_whitespace().skip(1);
    match args.next() {
        Some("cd") => Command::CD(args.next().unwrap()),
        Some("ls") => Command::LS,
        _ => panic!("Unknown command"),
    }
}

fn parse_filesystem(input: &str) -> HashMap<String, usize> {
    let lines = input.lines().peekable();

    let mut current_path = Vec::<&str>::new();
    let mut filesystem: HashMap<String, usize> = HashMap::new();
    for line in lines {
        if line.starts_with("$") {
            let cmd = parse_command_line(line); 
            match cmd {
                Command::CD(dir) => match dir {
                    "/" => current_path.clear(),
                    ".." => {
                        current_path.pop().unwrap();
                    },
                    dir => current_path.push(dir),
                },
                Command::LS => {}
            }
        } else if !line.starts_with("dir") {
            let (size, filename) = line.split_once(' ').unwrap();
            let file = File {
                name: String::from(filename),
                size: size.parse::<usize>().unwrap(),
            };
            for p in 0..(current_path.len() + 1) {
                let path = current_path[..p].join("/").to_string();

                filesystem.entry(path).and_modify(|s| *s += file.size).or_insert(file.size);
            }
        }
    }
    filesystem
}

