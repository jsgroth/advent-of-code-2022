//! Day 12: Hill Climbing Algorithm
//! https://adventofcode.com/2022/day/12

use std::cmp;
use std::collections::VecDeque;

struct Input {
    grid: Vec<Vec<u8>>,
    start: (usize, usize),
    end: (usize, usize),
}

struct Path {
    distance: usize,
    position: (usize, usize),
}

impl Path {
    fn new(distance: usize, position: (usize, usize)) -> Self {
        Self { distance, position }
    }
}

fn solve(grid: &Vec<Vec<u8>>, start: (usize, usize), end: (usize, usize)) -> Option<usize> {
    let rows = grid.len();
    let cols = grid[0].len();

    let mut visited = vec![vec![false; cols]; rows];
    visited[start.0][start.1] = true;

    let mut queue: VecDeque<Path> = VecDeque::new();
    queue.push_back(Path::new(0, start));

    while !queue.is_empty() {
        let Path { distance, position } = queue.pop_front().unwrap();

        if position == end {
            return Some(distance);
        }

        let (i, j) = position;
        for (dx, dy) in [(1, 0), (-1, 0), (0, 1), (0, -1)] {
            let ii = (i as i32) + dx;
            let jj = (j as i32) + dy;
            if ii < 0 || jj < 0 {
                continue;
            }

            let ii = ii as usize;
            let jj = jj as usize;
            if ii >= rows || jj >= cols {
                continue;
            }

            if grid[ii][jj] <= grid[i][j] + 1 && !visited[ii][jj] {
                visited[ii][jj] = true;
                queue.push_back(Path::new(distance + 1, (ii, jj)));
            }
        }
    }

    None
}

fn solve_part_1(input: &str) -> usize {
    let Input { grid, start, end } = parse_input(input);

    solve(&grid, start, end).expect("there should be a solution")
}

fn solve_part_2(input: &str) -> usize {
    let Input { grid, end, .. } = parse_input(input);

    let mut min_distance = usize::MAX;
    for (i, row) in grid.iter().enumerate() {
        for (j, &height) in row.iter().enumerate() {
            if height == 0 {
                if let Some(distance) = solve(&grid, (i, j), end) {
                    min_distance = cmp::min(min_distance, distance)
                }
            }
        }
    }

    min_distance
}

fn parse_input(input: &str) -> Input {
    let mut start = (usize::MAX, usize::MAX);
    let mut end = (usize::MAX, usize::MAX);

    let mut grid: Vec<Vec<u8>> = Vec::new();
    for (i, line) in input.lines().enumerate() {
        let mut row: Vec<u8> = Vec::new();
        for (j, c) in line.chars().enumerate() {
            match c {
                'S' => {
                    start = (i, j);
                    row.push(0);
                }
                'E' => {
                    end = (i, j);
                    row.push(('z' as u8) - ('a' as u8));
                }
                _ => {
                    row.push((c as u8) - ('a' as u8));
                }
            }
        }
        grid.push(row);
    }

    assert_ne!(start.0, usize::MAX);
    assert_ne!(end.0, usize::MAX);

    Input { grid, start, end }
}

fn main() {
    let input = advent_of_code_2022::read_input().expect("unable to read input file");

    let solution1 = solve_part_1(&input);
    println!("{solution1}");

    let solution2 = solve_part_2(&input);
    println!("{solution2}");
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_INPUT: &str = include_str!("sample_input/sample12.txt");

    #[test]
    fn test_sample_input_part_1() {
        assert_eq!(31, solve_part_1(SAMPLE_INPUT));
    }

    #[test]
    fn test_sample_input_part_2() {
        assert_eq!(29, solve_part_2(SAMPLE_INPUT));
    }
}