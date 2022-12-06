//! Day 6: Tuning Trouble
//! https://adventofcode.com/2022/day/6

use std::collections::HashSet;

const PART_1_WINDOW_SIZE: usize = 4;
const PART_2_WINDOW_SIZE: usize = 14;

fn solve(input: &str, window_size: usize) -> usize {
    let line = input.lines().next().expect("input should have a line");

    let chars: Vec<_> = line.chars().collect();
    chars.windows(window_size).enumerate().find_map(|(i, window)| {
        let window_chars: HashSet<_> = window.iter().copied().collect();
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

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_INPUT: &str = "mjqjpqmgbljsphdztnvjfqwrcgsmlb";

    #[test]
    fn test_sample_input_part_1() {
        assert_eq!(7, solve(SAMPLE_INPUT, PART_1_WINDOW_SIZE));
    }

    #[test]
    fn test_sample_input_part_2() {
        assert_eq!(19, solve(SAMPLE_INPUT, PART_2_WINDOW_SIZE));
    }

    #[test]
    fn additional_sample_tests_part_1() {
        assert_eq!(5, solve("bvwbjplbgvbhsrlpgdmjqwftvncz", PART_1_WINDOW_SIZE));
        assert_eq!(6, solve("nppdvjthqldpwncqszvftbrmjlhg", PART_1_WINDOW_SIZE));
        assert_eq!(10, solve("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", PART_1_WINDOW_SIZE));
        assert_eq!(11, solve("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", PART_1_WINDOW_SIZE));
    }

    #[test]
    fn additional_sample_tests_part_2() {
        assert_eq!(23, solve("bvwbjplbgvbhsrlpgdmjqwftvncz", PART_2_WINDOW_SIZE));
        assert_eq!(23, solve("nppdvjthqldpwncqszvftbrmjlhg", PART_2_WINDOW_SIZE));
        assert_eq!(29, solve("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", PART_2_WINDOW_SIZE));
        assert_eq!(26, solve("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", PART_2_WINDOW_SIZE));
    }
}