//! Day 23: Unstable Diffusion
//! https://adventofcode.com/2022/day/23

use std::cmp;
use std::collections::{HashMap, HashSet};

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    fn nw(&self) -> Self {
        Self { x: self.x - 1, y: self.y - 1 }
    }

    fn n(&self) -> Self {
        Self { x: self.x, y: self.y - 1 }
    }

    fn ne(&self) -> Self {
        Self { x: self.x + 1, y: self.y - 1 }
    }

    fn w(&self) -> Self {
        Self { x: self.x - 1, y: self.y }
    }

    fn e(&self) -> Self {
        Self { x: self.x + 1, y: self.y }
    }

    fn sw(&self) -> Self {
        Self { x: self.x - 1, y: self.y + 1 }
    }

    fn s(&self) -> Self {
        Self { x: self.x, y: self.y + 1 }
    }

    fn se(&self) -> Self {
        Self { x: self.x + 1, y: self.y + 1 }
    }

    fn all_adjacent(&self) -> Vec<Self> {
        vec![self.nw(), self.n(), self.ne(), self.w(), self.e(), self.sw(), self.s(), self.se()]
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Direction {
    North,
    South,
    West,
    East,
}

impl Direction {
    fn get_directions_shifted(iteration: usize) -> Vec<Self> {
        match iteration % 4 {
            0 => vec![Self::North, Self::South, Self::West, Self::East],
            1 => vec![Self::South, Self::West, Self::East, Self::North],
            2 => vec![Self::West, Self::East, Self::North, Self::South],
            3 => vec![Self::East, Self::North, Self::South, Self::West],
            _ => panic!("should never happen, i={iteration}"),
        }
    }
}

fn solve(input: &str) -> i32 {
    let mut elf_positions = parse_input(input);

    for iteration in 0..10 {
        elf_positions = simulate_iteration(iteration, elf_positions);
    }

    let (min_x, min_y) = get_minimums(&elf_positions);
    let (max_x, max_y) = get_maximums(&elf_positions);

    (max_x - min_x + 1) * (max_y - min_y + 1) - elf_positions.len() as i32
}

fn solve_part_2(input: &str) -> usize {
    let mut elf_positions = parse_input(input);

    for iteration in 0.. {
        let prev_positions = elf_positions.clone();
        elf_positions = simulate_iteration(iteration, elf_positions);
        if prev_positions == elf_positions {
            return iteration + 1;
        }
    }

    panic!("no solution found");
}

fn simulate_iteration(iteration: usize, elf_positions: HashSet<Point>) -> HashSet<Point> {
    let directions = Direction::get_directions_shifted(iteration);

    let mut proposed_new_positions: HashMap<Point, Point> = HashMap::with_capacity(elf_positions.len());

    for elf in &elf_positions {
        if !elf.all_adjacent().into_iter().any(|p| elf_positions.contains(&p)) {
            // No adjacent elves
            proposed_new_positions.insert(elf.clone(), elf.clone());
            continue;
        }

        let mut proposed_new_position = elf.clone();
        for direction in &directions {
            match direction {
                Direction::North => {
                    let adjacent = vec![elf.nw(), elf.n(), elf.ne()];
                    if !adjacent.into_iter().any(|p| elf_positions.contains(&p)) {
                        proposed_new_position = elf.n();
                        break;
                    }
                }
                Direction::South => {
                    let adjacent = vec![elf.sw(), elf.s(), elf.se()];
                    if !adjacent.into_iter().any(|p| elf_positions.contains(&p)) {
                        proposed_new_position = elf.s();
                        break;
                    }
                }
                Direction::West => {
                    let adjacent = vec![elf.nw(), elf.w(), elf.sw()];
                    if !adjacent.into_iter().any(|p| elf_positions.contains(&p)) {
                        proposed_new_position = elf.w();
                        break;
                    }
                }
                Direction::East => {
                    let adjacent = vec![elf.ne(), elf.e(), elf.se()];
                    if !adjacent.into_iter().any(|p| elf_positions.contains(&p)) {
                        proposed_new_position = elf.e();
                        break;
                    }
                }
            }
        }

        proposed_new_positions.insert(elf.clone(), proposed_new_position);
    }

    let mut proposed_position_counts: HashMap<Point, usize> = HashMap::new();
    for proposed_new_position in proposed_new_positions.values() {
        if let Some(count) = proposed_position_counts.get_mut(proposed_new_position) {
            *count += 1;
        } else {
            proposed_position_counts.insert(proposed_new_position.clone(), 1);
        }
    }

    for (elf, proposed_new_position) in proposed_new_positions.iter_mut() {
        if proposed_position_counts.get(proposed_new_position).copied().unwrap() > 1 {
            *proposed_new_position = elf.clone();
        }
    }

    proposed_new_positions.into_values().collect()
}

fn get_minimums(positions: &HashSet<Point>) -> (i32, i32) {
    positions.iter().fold((i32::MAX, i32::MAX), |(min_x, min_y), point| {
        (cmp::min(min_x, point.x), cmp::min(min_y, point.y))
    })
}

fn get_maximums(positions: &HashSet<Point>) -> (i32, i32) {
    positions.iter().fold((i32::MIN, i32::MIN), |(max_x, max_y), point| {
        (cmp::max(max_x, point.x), cmp::max(max_y, point.y))
    })
}

fn parse_input(input: &str) -> HashSet<Point> {
    let mut elf_positions: HashSet<Point> = HashSet::new();

    for (i, line) in input.lines().enumerate() {
        for (j, c) in line.chars().enumerate() {
            if c == '#' {
                elf_positions.insert(Point::new(j as i32, i as i32));
            }
        }
    }

    elf_positions
}

fn main() {
    let input = advent_of_code_2022::read_input().expect("unable to read input file");

    let solution1 = solve(&input);
    println!("{solution1}");

    let solution2 = solve_part_2(&input);
    println!("{solution2}");
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_INPUT: &str = include_str!("sample_input/sample23.txt");

    #[test]
    fn test_sample_input_part_1() {
        assert_eq!(110, solve(SAMPLE_INPUT));
    }

    #[test]
    fn test_sample_input_part_2() {
        assert_eq!(20, solve_part_2(SAMPLE_INPUT));
    }
}