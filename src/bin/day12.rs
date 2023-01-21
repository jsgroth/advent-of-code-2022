//! Day 12: Hill Climbing Algorithm
//! https://adventofcode.com/2022/day/12

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

fn solve(grid: &Vec<Vec<u8>>, start: Option<(usize, usize)>, end: (usize, usize)) -> usize {
    let rows = grid.len();
    let cols = grid[0].len();

    let mut visited = vec![vec![false; cols]; rows];
    visited[end.0][end.1] = true;

    let mut queue: VecDeque<Path> = VecDeque::new();
    queue.push_back(Path::new(0, end));

    while !queue.is_empty() {
        let Path { distance, position } = queue.pop_front().unwrap();

        let (i, j) = position;
        for (dx, dy) in [(1, 0), (-1, 0), (0, 1), (0, -1)] {
            if (i == 0 && dx == -1) || (j == 0 && dy == -1) {
                continue;
            }

            let ii = ((i as i32) + dx) as usize;
            let jj = ((j as i32) + dy) as usize;
            if ii >= rows || jj >= cols {
                continue;
            }

            if grid[i][j] <= grid[ii][jj] + 1 {
                match start {
                    Some(start) => {
                        if (ii, jj) == start {
                            return distance + 1;
                        }
                    }
                    None => {
                        if grid[ii][jj] == 0 {
                            return distance + 1;
                        }
                    }
                }

                if !visited[ii][jj] {
                    visited[ii][jj] = true;
                    queue.push_back(Path::new(distance + 1, (ii, jj)));
                }
            }
        }
    }

    panic!("no solution found");
}

fn solve_part_1(input: &str) -> usize {
    let Input { grid, start, end } = parse_input(input);

    solve(&grid, Some(start), end)
}

fn solve_part_2(input: &str) -> usize {
    let Input { grid, end, .. } = parse_input(input);

    solve(&grid, None, end)
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
                    row.push(b'z' - b'a');
                }
                _ => {
                    row.push((c as u8) - b'a');
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
