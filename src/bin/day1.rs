//! Day 1: Calorie Counting
//! https://adventofcode.com/2022/day/1

fn solve(input: &str) -> i32 {
    let lines: Vec<_> = input.lines().collect();
    lines.split(|s| s.is_empty())
        .map(parse_and_sum)
        .max()
        .expect("list should not be empty")
}

fn solve_part_2(input: &str) -> i32 {
    let lines: Vec<_> = input.lines().collect();
    let mut sums: Vec<_> = lines.split(|s| s.is_empty())
        .map(parse_and_sum)
        .collect();

    sums.sort_by(|a, b| a.cmp(b).reverse());

    sums[..3].iter().sum()
}

fn parse_and_sum(slice: &[&str]) -> i32 {
    slice.iter()
        .map(|s| s.parse::<i32>().expect("s should be an integer"))
        .sum()
}

fn main() {
    let input = advent_of_code_2022::read_input().expect("unable to read input file");

    let solution1 = solve(&input);
    println!("{solution1}");

    let solution2 = solve_part_2(&input);
    println!("{solution2}");
}