use std::cmp;
use std::fmt;

#[derive(Clone, Copy)]
struct Group {
    min: i32,
    max: i32,
}

impl Group {
    fn contains(&self, other: Group) -> bool {
        self.min <= other.min && self.max >= other.max
    }

    fn intersection(&self, other: Group) -> Group {
        let mut intersection_group = Group {
            min: cmp::max(self.min, other.min),
            max: cmp::min(self.max, other.max),
        };
        if intersection_group.min > intersection_group.max {
            intersection_group.min = -1;
            intersection_group.max = -1;
        }
        intersection_group
    }
}

impl fmt::Display for Group {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{}, {}]", self.min, self.max)
    }
}


fn get_group_from_string(groupstr: &str) -> Group {
    let minmax: Vec<&str> = groupstr.split("-").collect();
    let group = Group {
        min: minmax[0].parse().unwrap(), 
        max: minmax[1].parse().unwrap(),
    };
    group
}

fn parse_groups(line: &str) -> (Group, Group) {
    let groups: Vec<&str> = line.split(",").collect();
    (get_group_from_string(groups[0]), get_group_from_string(groups[1]))
}

fn find_fully_overlap(groups: (Group, Group)) -> bool {
    groups.0.contains(groups.1) || groups.1.contains(groups.0)
}

fn find_partly_overlap(groups: (Group, Group)) -> bool {
    let overlapping_group = groups.0.intersection(groups.1);
    overlapping_group.min != -1 && overlapping_group.max != -1
}

fn get_n_fully_contains(input: &str) -> usize {
    input
        .lines()
        .map(|line| find_fully_overlap(parse_groups(line)) as usize)
        .sum()
}

fn get_n_partly_overloap(input: &str) -> usize {
    input
        .lines()
        .map(|line| find_partly_overlap(parse_groups(line)) as usize)
        .sum()
}

fn main() {
    let input = include_str!("input.txt");
    let n_fully_contains = get_n_fully_contains(input);
    let n_partly_contains = get_n_partly_overloap(input);
    println!("n_fully_contains = {}", n_fully_contains);
    println!("n_partly_contains = {}", n_partly_contains);
}


#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = "2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8";

    #[test]
    fn test_get_group_from_string() {
        let mut groupstr = EXAMPLE_INPUT.lines().collect::<Vec<_>>()[0];
        groupstr = groupstr.split(",").collect::<Vec<_>>()[0];
        let group = get_group_from_string(groupstr);
        assert_eq!(group.min, 2);
        assert_eq!(group.max, 4);
    }

    #[test]
    fn test_parse_groups() {
        let groups = EXAMPLE_INPUT
            .lines()
            .map(parse_groups)
            .collect::<Vec<_>>();
        assert_eq!(groups[0].0.min, 2);
        assert_eq!(groups[0].0.max, 4);
        assert_eq!(groups[0].1.min, 6);
        assert_eq!(groups[0].1.max, 8);
    }

    #[test]
    fn test_find_fully_overlap() {
        let group0 = Group {min: 2, max: 5};
        let group1 = Group {min: 3, max: 6};
        let group2 = Group {min: 3, max: 4};
        assert_eq!(find_fully_overlap((group0, group1)), false);
        assert_eq!(find_fully_overlap((group0, group2)), true);
        assert_eq!(find_fully_overlap((group1, group2)), true);
    }

    #[test]
    fn test_find_partly_overlap() {
        let group0 = Group {min: 2, max: 5};
        let group1 = Group {min: 6, max: 9};
        let group2 = Group {min: 5, max: 6};
        assert_eq!(find_partly_overlap((group0, group1)), false);
        assert_eq!(find_partly_overlap((group0, group2)), true);
        assert_eq!(find_partly_overlap((group1, group2)), true);
    }

    #[test]
    fn test_get_n_fully_contains() {
        let sum = get_n_fully_contains(EXAMPLE_INPUT);
        assert_eq!(sum, 2);
    }
}