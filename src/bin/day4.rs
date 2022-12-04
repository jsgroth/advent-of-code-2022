//! Day 4: Camp Cleanup
//! https://adventofcode.com/2022/day/4

struct Range {
    left: u32,
    right: u32,
}

impl Range {
    fn from_str(s: &str) -> Range {
        let (left, right) = s.split_once('-').expect("range should have one dash");

        let left: u32 = left.parse().expect("left end should be an integer");
        let right: u32 = right.parse().expect("right end should be an integer");

        Range { left, right }
    }

    fn fully_contains(&self, other: &Range) -> bool {
        self.left <= other.left && self.right >= other.right
    }

    fn overlaps(&self, other: &Range) -> bool {
        !(self.right < other.left || self.left > other.right)
    }
}

fn solve(input: &str) -> usize {
    input.lines()
        .map(parse_input_line)
        .filter(|(left_range, right_range)| {
            left_range.fully_contains(right_range) || right_range.fully_contains(left_range)
        })
        .count()
}

fn solve_part_2(input: &str) -> usize {
    input.lines()
        .map(parse_input_line)
        .filter(|(left_range, right_range)| {
            left_range.overlaps(right_range)
        })
        .count()
}

fn parse_input_line(line: &str) -> (Range, Range) {
    let (left_range, right_range) = line.split_once(',').expect("line should have one comma");

    (Range::from_str(left_range), Range::from_str(right_range))
}

fn main() {
    let input = advent_of_code_2022::read_input().expect("unable to read input file");

    let solution1 = solve(&input);
    println!("{solution1}");

    let solution2 = solve_part_2(&input);
    println!("{solution2}");
}