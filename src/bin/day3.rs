//! Day 3: Rucksack Reorganization
//! https://adventofcode.com/2022/day/3

use std::collections::HashSet;

fn solve(input: &str) -> u32 {
    input.lines().map(|line| {
        let (lhalf, rhalf) = line.split_at(line.len() / 2);

        let lchars: HashSet<_> = lhalf.chars().collect();

        let c = rhalf.chars().find(|c| {
            lchars.contains(c)
        }).expect("there should be a character in both l and r");

        priority(c)
    })
        .sum()
}

fn solve_part_2(input: &str) -> u32 {
    let lines: Vec<_> = input.lines().collect();
    lines.chunks(3).map(|chunk| {
        let [a, b, c] = match chunk {
            [a, b, c] => [a, b, c],
            _ => panic!("the number of lines should be a multiple of 3"),
        };

        let achars: HashSet<_> = a.chars().collect();
        let bchars: HashSet<_> = b.chars().collect();

        let ch = c.chars().find(|ch| {
            achars.contains(ch) && bchars.contains(ch)
        }).expect("there should a be a character in all three lines");

        priority(ch)
    })
        .sum()
}

fn priority(c: char) -> u32 {
    if c.is_lowercase() {
        (c as u32) - ('a' as u32) + 1
    } else {
        (c as u32) - ('A' as u32) + 27
    }
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

    const SAMPLE_INPUT: &str = "\
vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw
";

    #[test]
    fn test_sample_input_part_1() {
        assert_eq!(157, solve(SAMPLE_INPUT));
    }

    #[test]
    fn test_sample_input_part_2() {
        assert_eq!(70, solve_part_2(SAMPLE_INPUT));
    }
}