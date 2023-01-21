//! Day 14: Regolith Reservoir
//! https://adventofcode.com/2022/day/14

use std::collections::HashSet;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Point {
    x: i32,
    y: i32,
}

struct SparseGrid {
    grid: HashSet<Point>,
}

impl SparseGrid {
    fn new() -> Self {
        Self {
            grid: HashSet::new(),
        }
    }

    fn get(&self, x: i32, y: i32) -> bool {
        self.grid.contains(&Point { x, y })
    }

    fn set(&mut self, x: i32, y: i32) {
        self.grid.insert(Point { x, y });
    }

    fn max_row(&self) -> Option<i32> {
        self.grid.iter().map(|Point { y, .. }| y).max().copied()
    }
}

const POUR_START_X: i32 = 500;
const POUR_START_Y: i32 = 0;

fn solve(input: &str, inf_rock_at_bottom: bool) -> usize {
    let mut sparse_grid = parse_input(input);
    assert!(!sparse_grid.grid.is_empty());

    let max_row = if inf_rock_at_bottom {
        sparse_grid.max_row().unwrap() + 1
    } else {
        sparse_grid.max_row().unwrap()
    };

    let mut sand_count = 0;
    loop {
        let mut sand_x = POUR_START_X;
        let mut sand_y = POUR_START_Y;

        while sand_y < max_row {
            if !sparse_grid.get(sand_x, sand_y + 1) {
                sand_y += 1;
            } else if !sparse_grid.get(sand_x - 1, sand_y + 1) {
                sand_y += 1;
                sand_x -= 1;
            } else if !sparse_grid.get(sand_x + 1, sand_y + 1) {
                sand_y += 1;
                sand_x += 1;
            } else {
                break;
            }
        }

        if !inf_rock_at_bottom && sand_y == max_row {
            // Fell into the abyss
            break;
        }

        sparse_grid.set(sand_x, sand_y);
        sand_count += 1;

        if inf_rock_at_bottom && sand_x == POUR_START_X && sand_y == POUR_START_Y {
            break;
        }
    }

    sand_count
}

fn parse_input(input: &str) -> SparseGrid {
    let paths: Vec<_> = input.lines().map(parse_line).collect();

    let mut sparse_grid = SparseGrid::new();
    for path in &paths {
        for window in path.windows(2) {
            let (mut p0_x, mut p0_y) = window[0];
            let (p1_x, p1_y) = window[1];

            sparse_grid.set(p0_x, p0_y);
            while p0_x != p1_x || p0_y != p1_y {
                p0_x += (p1_x - p0_x).signum();
                p0_y += (p1_y - p0_y).signum();

                sparse_grid.set(p0_x, p0_y);
            }
        }
    }

    sparse_grid
}

fn parse_line(line: &str) -> Vec<(i32, i32)> {
    line.split(" -> ")
        .map(|point| {
            let (x, y) = point.split_once(',').expect("point should have one comma");
            let x: i32 = x.parse().expect("point x should be an integer");
            let y: i32 = y.parse().expect("point y should be an integer");
            (x, y)
        })
        .collect()
}

fn main() {
    let input = advent_of_code_2022::read_input().expect("unable to read input file");

    let solution1 = solve(&input, false);
    println!("{solution1}");

    let solution2 = solve(&input, true);
    println!("{solution2}");
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_INPUT: &str = include_str!("sample_input/sample14.txt");

    #[test]
    fn test_sample_input_part_1() {
        assert_eq!(24, solve(SAMPLE_INPUT, false));
    }

    #[test]
    fn test_sample_input_part_2() {
        assert_eq!(93, solve(SAMPLE_INPUT, true));
    }
}
