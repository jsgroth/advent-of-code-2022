//! Day 6: Tuning Trouble
//! https://adventofcode.com/2022/day/6

use std::collections::HashSet;

const PART_1_WINDOW_SIZE: usize = 4;
const PART_2_WINDOW_SIZE: usize = 14;

fn solve(input: &str, window_size: usize) -> usize {
    let line = input.lines().next().expect("input should have a line");

    let chars: Vec<_> = line.chars().collect();
    chars.windows(window_size).enumerate().find_map(|(i, window)| {
        let window_chars: HashSet<char> = HashSet::from_iter(window.iter().copied());
        if window_chars.len() == window_size {
            Some(i + window_size)
        } else {
            None
        }
    })
        .expect("no solution found")
}

fn main() {
    let input = advent_of_code_2022::read_input().expect("unable to read input file");

    let solution1 = solve(&input, PART_1_WINDOW_SIZE);
    println!("{solution1}");

    let solution2 = solve(&input, PART_2_WINDOW_SIZE);
    println!("{solution2}");
}