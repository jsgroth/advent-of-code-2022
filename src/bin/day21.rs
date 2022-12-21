//! Day 21: Monkey Math
//! https://adventofcode.com/2022/day/21

use std::collections::{HashMap, HashSet};

#[derive(Debug)]
enum Monkey {
    Constant { name: String, n: i64 },
    Add { name: String, a: Box<Monkey>, b: Box<Monkey> },
    Subtract { name: String, a: Box<Monkey>, b: Box<Monkey> },
    Multiply { name: String, a: Box<Monkey>, b: Box<Monkey> },
    Divide { name: String, a: Box<Monkey>, b: Box<Monkey> },
}

impl Monkey {
    fn from_line(name: &str, line: &str, all_lines: &HashMap<&str, &str>) -> Self {
        let name = String::from(name);

        let split: Vec<_> = line.split(' ').collect();
        match split.as_slice() {
            [n] => {
                Self::Constant { name, n: n.parse().expect("single word should be an integer") }
            }
            [a, op, b] => {
                let a_monkey = Self::from_line(a, all_lines.get(a).unwrap(), all_lines);
                let b_monkey = Self::from_line(b, all_lines.get(b).unwrap(), all_lines);
                match *op {
                    "+" => Self::Add { name, a: Box::new(a_monkey), b: Box::new(b_monkey) },
                    "-" => Self::Subtract { name, a: Box::new(a_monkey), b: Box::new(b_monkey) },
                    "*" => Self::Multiply { name, a: Box::new(a_monkey), b: Box::new(b_monkey) },
                    "/" => Self::Divide { name, a: Box::new(a_monkey), b: Box::new(b_monkey) },
                    _ => panic!("unexpected operator: {op}"),
                }
            }
            _ => panic!("unexpected line format: {line}"),
        }
    }

    fn evaluate(&self) -> i64 {
        match self {
            Self::Constant { n, .. } => *n,
            Self::Add { a, b, .. } => a.evaluate() + b.evaluate(),
            Self::Subtract { a, b, .. } => a.evaluate() - b.evaluate(),
            Self::Multiply { a, b, .. } => a.evaluate() * b.evaluate(),
            Self::Divide { a, b, .. } => a.evaluate() / b.evaluate(),
        }
    }

    fn find_human_path<'a>(&'a self, human_path: &mut HashSet<&'a str>) -> bool {
        if self.get_name() == "humn" {
            human_path.insert("humn");
            return true;
        }

        match self {
            Self::Constant { .. } => false,
            Self::Add { name, a, b } |
            Self::Subtract { name, a, b } |
            Self::Multiply { name, a, b } |
            Self::Divide { name, a, b } => {
                if a.find_human_path(human_path) || b.find_human_path(human_path) {
                    human_path.insert(name.as_str());
                    true
                } else {
                    false
                }
            }
        }
    }

    fn solve_for_human(&self, current_value: i64, human_path: &HashSet<&str>) -> i64 {
        match self {
            Self::Constant { name, n } => {
                if human_path.contains(name.as_str()) {
                    current_value
                } else {
                    *n
                }
            }
            Self::Add { a, b, .. } => {
                if human_path.contains(a.get_name()) {
                    a.solve_for_human(current_value - b.evaluate(), human_path)
                } else {
                    b.solve_for_human(current_value - a.evaluate(), human_path)
                }
            }
            Self::Subtract { a, b, .. } => {
                if human_path.contains(a.get_name()) {
                    a.solve_for_human(current_value + b.evaluate(), human_path)
                } else {
                    b.solve_for_human(a.evaluate() - current_value, human_path)
                }
            }
            Self::Multiply { a, b, .. } => {
                if human_path.contains(a.get_name()) {
                    a.solve_for_human(current_value / b.evaluate(), human_path)
                } else {
                    b.solve_for_human(current_value / a.evaluate(), human_path)
                }
            }
            Self::Divide { a, b, .. } => {
                if human_path.contains(a.get_name()) {
                    a.solve_for_human(current_value * b.evaluate(), human_path)
                } else {
                    b.solve_for_human(a.evaluate() / current_value, human_path)
                }
            }
        }
    }

    fn get_name(&self) -> &str {
        match self {
            Self::Constant { name, .. } |
            Self::Add { name, .. } |
            Self::Subtract { name, .. } |
            Self::Multiply { name, .. } |
            Self::Divide { name, .. } => {
                name
            }
        }
    }
}

fn solve(input: &str) -> i64 {
    let root_monkey = parse_input(input);

    root_monkey.evaluate()
}

fn solve_part_2(input: &str) -> i64 {
    let root_monkey = parse_input(input);

    let mut human_path: HashSet<&str> = HashSet::new();
    root_monkey.find_human_path(&mut human_path);

    match &root_monkey {
        Monkey::Add { a, b, .. } | Monkey::Subtract { a, b, .. } |
        Monkey::Multiply { a, b, .. } | Monkey::Divide { a, b, .. } => {
            if human_path.contains(a.get_name()) {
                a.solve_for_human(b.evaluate(), &human_path)
            } else {
                b.solve_for_human(a.evaluate(), &human_path)
            }
        }
        _ => panic!("root monkey should not be a constant")
    }
}

fn parse_input(input: &str) -> Monkey {
    let lines_by_name: HashMap<_, _> = input.lines().map(|line| {
        let (name, rest_of_line) = line.split_once(' ').expect("every line should have a space");
        let name = &name[..name.len() - 1];

        (name, rest_of_line)
    })
        .collect();

    let root_line = lines_by_name.get("root").expect("input should have a root line");
    Monkey::from_line("root", root_line, &lines_by_name)
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

    const SAMPLE_INPUT: &str = include_str!("sample_input/sample21.txt");

    #[test]
    fn test_sample_input_part_1() {
        assert_eq!(152, solve(SAMPLE_INPUT));
    }

    #[test]
    fn test_sample_input_part_2() {
        assert_eq!(301, solve_part_2(SAMPLE_INPUT));
    }
}