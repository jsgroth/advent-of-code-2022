//! Day 9: Rope Bridge
//!
//! <https://adventofcode.com/2022/day/9>

use std::collections::HashSet;

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    fn is_adjacent_to(&self, other: &Self) -> bool {
        (self.x - other.x).abs() <= 1 && (self.y - other.y).abs() <= 1
    }
}

const PART_1_ROPE_LEN: usize = 2;
const PART_2_ROPE_LEN: usize = 10;

fn solve(input: &str, rope_len: usize) -> usize {
    let mut knots = vec![Point::new(0, 0); rope_len];

    let mut tail_visited: HashSet<Point> = HashSet::new();
    tail_visited.insert(Point::new(0, 0));

    for line in input.lines() {
        let (direction, distance) = line
            .split_once(' ')
            .expect("every line should have a space");
        let distance: i32 = distance.parse().expect("distance should be an integer");

        let (dx, dy) = parse_direction(direction);

        for _ in 0..distance {
            knots[0].x += dx;
            knots[0].y += dy;

            for i in 1..knots.len() {
                if !knots[i - 1].is_adjacent_to(&knots[i]) {
                    knots[i] = move_tail(&knots[i - 1], &knots[i]);
                }
            }

            tail_visited.insert(knots.last().unwrap().clone());
        }
    }

    tail_visited.len()
}

fn parse_direction(direction: &str) -> (i32, i32) {
    match direction {
        "R" => (1, 0),
        "L" => (-1, 0),
        "U" => (0, 1),
        "D" => (0, -1),
        _ => panic!("direction should be R/L/U/D"),
    }
}

fn move_tail(head: &Point, tail: &Point) -> Point {
    let new_x = tail.x + (head.x - tail.x).signum();
    let new_y = tail.y + (head.y - tail.y).signum();
    Point::new(new_x, new_y)
}

fn main() {
    let input = advent_of_code_2022::read_input().expect("unable to read input file");

    let solution1 = solve(&input, PART_1_ROPE_LEN);
    println!("{solution1}");

    let solution2 = solve(&input, PART_2_ROPE_LEN);
    println!("{solution2}");
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_INPUT: &str = include_str!("sample_input/sample9.txt");
    const LARGER_SAMPLE_INPUT: &str = include_str!("sample_input/sample9-larger.txt");

    #[test]
    fn test_sample_input_part_1() {
        assert_eq!(13, solve(SAMPLE_INPUT, PART_1_ROPE_LEN));
    }

    #[test]
    fn test_sample_input_part_2() {
        assert_eq!(1, solve(SAMPLE_INPUT, PART_2_ROPE_LEN));
    }

    #[test]
    fn test_larger_sample_input_part_2() {
        assert_eq!(36, solve(LARGER_SAMPLE_INPUT, PART_2_ROPE_LEN));
    }
}
