//! Day 24: Blizzard Basin
//!
//! <https://adventofcode.com/2022/day/24>

use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashSet};

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Direction {
    Up,
    Left,
    Right,
    Down,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct Blizzard {
    direction: Direction,
    i: usize,
    j: usize,
}

impl Blizzard {
    fn new(direction: Direction, i: usize, j: usize) -> Self {
        Self { direction, i, j }
    }

    fn move_once(&self, rows: usize, cols: usize) -> Self {
        match self.direction {
            Direction::Up => {
                let i = if self.i == 1 { rows - 2 } else { self.i - 1 };
                Self::new(self.direction, i, self.j)
            }
            Direction::Left => {
                let j = if self.j == 1 { cols - 2 } else { self.j - 1 };
                Self::new(self.direction, self.i, j)
            }
            Direction::Right => {
                let j = if self.j == cols - 2 { 1 } else { self.j + 1 };
                Self::new(self.direction, self.i, j)
            }
            Direction::Down => {
                let i = if self.i == rows - 2 { 1 } else { self.i + 1 };
                Self::new(self.direction, i, self.j)
            }
        }
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct SearchState {
    elf_i: usize,
    elf_j: usize,
    iteration: usize,
    target_i: usize,
    target_j: usize,
}

impl SearchState {
    // Heuristic function for A* search, lower bound for lowest possible result from this state
    fn optimal_distance(&self) -> usize {
        let i_distance = (self.target_i as i32 - self.elf_i as i32).unsigned_abs() as usize;
        let j_distance = (self.target_j as i32 - self.elf_j as i32).unsigned_abs() as usize;
        self.iteration + i_distance + j_distance
    }
}

impl PartialOrd<Self> for SearchState {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for SearchState {
    fn cmp(&self, other: &Self) -> Ordering {
        // Reverse so that smaller ends up on top of heap
        self.optimal_distance()
            .cmp(&other.optimal_distance())
            .reverse()
    }
}

fn solve(input: &str) -> (usize, usize) {
    let initial_grid = parse_input(input);
    let rows = initial_grid.len();
    let cols = initial_grid[0].len();

    let initial_state = SearchState {
        elf_i: 0,
        elf_j: 1,
        iteration: 0,
        target_i: rows - 2,
        target_j: cols - 2,
    };
    let first_step = find_shortest_distance(&initial_grid, initial_state);

    let second_state = SearchState {
        elf_i: rows - 1,
        elf_j: cols - 2,
        iteration: first_step - 1,
        target_i: 1,
        target_j: 1,
    };
    let second_step = find_shortest_distance(&initial_grid, second_state);

    let final_state = SearchState {
        elf_i: 0,
        elf_j: 1,
        iteration: second_step - 1,
        target_i: rows - 2,
        target_j: cols - 2,
    };
    let final_step = find_shortest_distance(&initial_grid, final_state);

    (first_step, final_step)
}

fn find_shortest_distance(
    initial_grid: &Vec<Vec<Vec<Blizzard>>>,
    initial_state: SearchState,
) -> usize {
    let SearchState {
        target_i, target_j, ..
    } = initial_state;

    let rows = initial_grid.len();
    let cols = initial_grid[0].len();

    let mut grids = vec![initial_grid.clone()];
    let mut queue: BinaryHeap<SearchState> = BinaryHeap::new();
    queue.push(initial_state);

    let mut checked_states: HashSet<SearchState> = HashSet::new();

    while !queue.is_empty() {
        let SearchState {
            elf_i,
            elf_j,
            iteration,
            ..
        } = queue.pop().unwrap();

        while grids.len() - 1 < iteration + 1 {
            grids.push(move_blizzards(grids.last().unwrap()));
        }

        let next_grid = &grids[iteration + 1];

        for (dx, dy) in [(1, 0), (-1, 0), (0, 1), (0, -1), (0, 0)] {
            let new_i = elf_i as i32 + dy;
            let new_j = elf_j as i32 + dx;

            if (new_i == 0 && new_j != 1) || new_i < 0 || new_j <= 0 {
                continue;
            }

            let new_i = new_i as usize;
            let new_j = new_j as usize;
            if (new_i == rows - 1 && new_j != cols - 2) || new_i > rows - 1 || new_j >= cols - 1 {
                continue;
            }

            if !next_grid[new_i][new_j].is_empty() {
                continue;
            }

            if new_i == target_i && new_j == target_j {
                // +2 because it takes one more move to get into the corner from here
                return iteration + 2;
            }

            let new_state = SearchState {
                elf_i: new_i,
                elf_j: new_j,
                iteration: iteration + 1,
                target_i,
                target_j,
            };
            if !checked_states.contains(&new_state) {
                checked_states.insert(new_state.clone());
                queue.push(new_state);
            }
        }
    }

    panic!("no solution found");
}

fn move_blizzards(grid: &Vec<Vec<Vec<Blizzard>>>) -> Vec<Vec<Vec<Blizzard>>> {
    let rows = grid.len();
    let cols = grid[0].len();

    let mut new_grid: Vec<Vec<Vec<Blizzard>>> = vec![vec![Vec::new(); cols]; rows];
    for row in grid {
        for blizzards in row {
            for blizzard in blizzards {
                let new_blizzard = blizzard.move_once(rows, cols);
                new_grid[new_blizzard.i][new_blizzard.j].push(new_blizzard);
            }
        }
    }

    new_grid
}

fn parse_input(input: &str) -> Vec<Vec<Vec<Blizzard>>> {
    let rows = input.lines().count();
    let cols = input
        .lines()
        .next()
        .expect("input should not be empty")
        .len();

    let mut grid: Vec<Vec<Vec<Blizzard>>> = vec![vec![Vec::new(); cols]; rows];
    for (i, line) in input.lines().enumerate() {
        if i > 0 && i < rows - 1 {
            for (j, c) in line.chars().enumerate() {
                if j > 0 && j < cols - 1 && c != '.' {
                    let direction = match c {
                        '^' => Direction::Up,
                        '<' => Direction::Left,
                        '>' => Direction::Right,
                        'v' => Direction::Down,
                        _ => panic!("unexpected char: {c}"),
                    };
                    grid[i][j].push(Blizzard::new(direction, i, j));
                }
            }
        }
    }

    grid
}

fn main() {
    let input = advent_of_code_2022::read_input().expect("unable to read input file");

    let (solution1, solution2) = solve(&input);
    println!("{solution1}");
    println!("{solution2}");
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_INPUT: &str = include_str!("sample_input/sample24.txt");

    #[test]
    fn test_sample_input_part_1() {
        let (solution1, _) = solve(SAMPLE_INPUT);
        assert_eq!(18, solution1);
    }

    #[test]
    fn test_sample_input_part_2() {
        let (_, solution2) = solve(SAMPLE_INPUT);
        assert_eq!(54, solution2);
    }
}
