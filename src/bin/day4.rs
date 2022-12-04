//! Day 4: Camp Cleanup
//! https://adventofcode.com/2022/day/4

struct Range {
    left: u32,
    right: u32,
}

impl Range {
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
    let mut split = line.split(',');

    let left_range = split.next().expect("no line should be empty");
    let right_range = split.next().expect("every line should have a comma");

    (parse_range(left_range), parse_range(right_range))
}

fn parse_range(range: &str) -> Range {
    let mut split = range.split('-');

    let left: u32 = split.next().expect("range should not be empty string")
        .parse().expect("left end of range should be an integer");
    let right: u32 = split.next().expect("every range should have a dash")
        .parse().expect("right end of range should be an integer");

    Range { left, right }
}

fn main() {
    let input = advent_of_code_2022::read_input().expect("unable to read input file");

    let solution1 = solve(&input);
    println!("{solution1}");

    let solution2 = solve_part_2(&input);
    println!("{solution2}");
}