use std::collections::HashSet;

fn main() {
    let input = include_str!("input.txt");
    let raw_forest = get_raw_forest(input);
    let n_visible_trees = get_n_visible_trees_from_outside(&raw_forest);
    println!("{}", n_visible_trees);
    let best_scenic_score = get_best_scenic_score(&raw_forest);
    println!("{}", best_scenic_score);
}

fn get_raw_forest(input: &str) -> Vec<Vec<u32>> {
    let raw_forest = input
        .lines()
        .map(|line| 
                line
                    .chars()
                    .map(|c| c.to_digit(10).unwrap())
                    .collect::<Vec<_>>()
            ).collect::<Vec<_>>();
    raw_forest
}

fn get_n_visible_trees_from_outside(raw_forest: &Vec<Vec<u32>>) -> usize {
    let h = raw_forest.len();
    let w = raw_forest[0].len();
    let mut visible = HashSet::<(usize, usize)>::with_capacity(h * w);
    for i in 1..(h-1) {
        let mut highest = raw_forest[i][0];
        for j in 1..(w-1) {
            if raw_forest[i][j] > highest {
                highest = raw_forest[i][j];
                visible.insert((i, j));
            }
        }
        let mut highest = raw_forest[i][w-1];
        for j in (1..(w-1)).rev() {
            if raw_forest[i][j] > highest {
                highest = raw_forest[i][j];
                visible.insert((i, j));
            }
        }
    }

    for j in 1..(w-1) {
        let mut highest = raw_forest[0][j];
        for i in 1..(h-1) {
            if raw_forest[i][j] > highest {
                highest = raw_forest[i][j];
                visible.insert((i, j));
            }
        }

        let mut highest = raw_forest[h-1][j];
        for i in (1..(h-1)).rev() {
            if raw_forest[i][j] > highest {
                highest = raw_forest[i][j];
                visible.insert((i, j));
            }
        }
    }
    visible.len() + h * 2 + w * 2 - 4
}


fn get_scenic_score(forest: &Vec<Vec<u32>>, x: usize, y: usize) -> u32 {
    let h = forest.len();
    let w = forest[0].len();
    let mut left = 0;
    let mut right = 0;
    let mut top = 0;
    let mut bottom = 0;

    let height = forest[y][x];
    // Top
    for i in (0..y).rev() {
        top+=1;
        if forest[i][x] >= height {
            break;
        }
    }
    // Bottom
    for i in (y+1)..h {
        bottom+=1;
        if forest[i][x] >= height {
            break;
        }
    }
    // Left
    for j in (0..x).rev() {
        left+=1;
        if forest[y][j] >= height {
            break;
        }
    }
    // Right
    for j in (x+1)..w {
        right+=1;
        if forest[y][j] >= height {
            break;
        }
    }
    let score = left * right * top * bottom;
    score
}


fn get_best_scenic_score(raw_forest: &Vec<Vec<u32>>) -> u32 {
    let h = raw_forest.len();
    let w = raw_forest[0].len();
    let mut best: u32 = 0;
    for i in 1..(h-1) {
        for j in 1..(w-1) {
            best = get_scenic_score(&raw_forest, i, j).max(best);
        }
    }
    best
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = "30373
25512
65332
33549
35390";

    #[test]
    fn test_example_part1() {
        let map = get_raw_forest(EXAMPLE_INPUT);

        assert_eq!(get_n_visible_trees_from_outside(&map), 21);
    }

    // #[test]
    // fn test_example_part2() {
    //     let map = get_raw_forest(EXAMPLE_INPUT);

    //     assert_eq!(best_score(&map), 8);
    // }

    #[test]
    fn test_scenic_score() {
        let map = get_raw_forest(EXAMPLE_INPUT);

        assert_eq!(get_scenic_score(&map, 2, 1), 4);
        assert_eq!(get_scenic_score(&map, 2, 3), 8);
    }

    #[test]
    fn test_get_raw_forest() {
        let map = get_raw_forest(EXAMPLE_INPUT);

        assert_eq!(map.len(), 5);
        assert_eq!(map[0].len(), 5);
    }
}