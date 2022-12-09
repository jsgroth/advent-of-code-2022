//! Day 9: Rope Bridge
//! https://adventofcode.com/2022/day/9

use std::collections::{HashMap, HashSet};

#[derive(Debug, PartialEq, Eq, Hash)]
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

impl Clone for Point {
    fn clone(&self) -> Self {
        Self { x: self.x, y: self.y }
    }
}

const PART_2_ROPE_LEN: usize = 10;

fn solve(input: &str) -> usize {
    let mut head = Point::new(0, 0);
    let mut tail = Point::new(0, 0);

    let mut tail_visited: HashSet<Point> = HashSet::new();
    tail_visited.insert(Point::new(0, 0));

    for line in input.lines() {
        let (direction, distance) = line.split_once(' ').expect("every line should have a space");
        let distance: i32 = distance.parse().expect("distance should be an integer");

        let (dx, dy) = parse_direction(direction);

        for _ in 0..distance {
            head.x += dx;
            head.y += dy;

            if !head.is_adjacent_to(&tail) {
                move_tail(&head, &mut tail);
                tail_visited.insert(tail.clone());
            }
        }
    }

    tail_visited.len()
}

fn solve_part_2(input: &str) -> usize {
    let mut knots: Vec<Point> = Vec::with_capacity(PART_2_ROPE_LEN);
    for _ in 0..PART_2_ROPE_LEN {
        knots.push(Point::new(0, 0));
    }

    let mut tail_visited: HashSet<Point> = HashSet::new();
    tail_visited.insert(Point::new(0, 0));

    for line in input.lines() {
        let (direction, distance) = line.split_once(' ').expect("every line should have a space");
        let distance: i32 = distance.parse().expect("distance should be an integer");

        let (dx, dy) = parse_direction(direction);

        for _ in 0..distance {
            knots[0].x += dx;
            knots[0].y += dy;

            for i in 1..knots.len() {
                if !knots[i - 1].is_adjacent_to(&knots[i]) {
                    move_tail(&knots[i - 1].clone(), &mut knots[i]);
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

fn move_tail(head: &Point, tail: &mut Point) {
    tail.x += (head.x - tail.x).signum();
    tail.y += (head.y - tail.y).signum();
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
R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2
";

    const LARGER_SAMPLE_INPUT: &str = "\
R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20
";

    #[test]
    fn test_sample_input_part_1() {
        assert_eq!(13, solve(SAMPLE_INPUT));
    }

    #[test]
    fn test_sample_input_part_2() {
        assert_eq!(1, solve_part_2(SAMPLE_INPUT));
    }

    #[test]
    fn test_larger_sample_input_part_2() {
        assert_eq!(36, solve_part_2(LARGER_SAMPLE_INPUT));
    }
}