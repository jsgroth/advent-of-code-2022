//! Day 21: Monkey Math
//! https://adventofcode.com/2022/day/21

use std::collections::{HashMap, HashSet};

#[derive(Debug)]
enum Monkey {
    Constant(i64),
    Add(String, String),
    Subtract(String, String),
    Multiply(String, String),
    Divide(String, String),
}

impl Monkey {
    fn evaluate(&self, monkey_map: &HashMap<&str, &Monkey>) -> i64 {
        match self {
            Self::Constant(n) => *n,
            Self::Add(a, b) => {
                monkey_map.get(a.as_str()).unwrap().evaluate(monkey_map) + monkey_map.get(b.as_str()).unwrap().evaluate(monkey_map)
            }
            Self::Subtract(a, b) => {
                monkey_map.get(a.as_str()).unwrap().evaluate(monkey_map) - monkey_map.get(b.as_str()).unwrap().evaluate(monkey_map)
            }
            Self::Multiply(a, b) => {
                monkey_map.get(a.as_str()).unwrap().evaluate(monkey_map) * monkey_map.get(b.as_str()).unwrap().evaluate(monkey_map)
            }
            Self::Divide(a, b) => {
                monkey_map.get(a.as_str()).unwrap().evaluate(monkey_map) / monkey_map.get(b.as_str()).unwrap().evaluate(monkey_map)
            }
        }
    }

    fn populate_contains_human<'a>(&'a self, name: &'a str, monkey_map: &HashMap<&str, &'a Monkey>, contains_human: &mut HashSet<&'a str>) -> bool {
        match self {
            Self::Constant(_) => name == "humn",
            Self::Add(a, b) | Self::Subtract(a, b) | Self::Multiply(a, b) | Self::Divide(a, b) => {
                let a_contains_human = monkey_map.get(a.as_str()).unwrap().populate_contains_human(a, monkey_map, contains_human);
                let b_contains_human = monkey_map.get(b.as_str()).unwrap().populate_contains_human(b, monkey_map, contains_human);
                if a_contains_human || b_contains_human {
                    contains_human.insert(name);
                    true
                } else {
                    false
                }
            }
        }
    }

    fn solve(&self, n: i64, name: &str, monkey_map: &HashMap<&str, &Monkey>, contains_human: &HashSet<&str>) -> i64 {
        match self {
            Self::Constant(my_n) => {
                if name == "humn" {
                    n
                } else {
                    *my_n
                }
            }
            Self::Add(a, b) => {
                if contains_human.contains(a.as_str()) {
                    let n = n - monkey_map.get(b.as_str()).unwrap().evaluate(monkey_map);
                    monkey_map.get(a.as_str()).unwrap().solve(n, a, monkey_map, contains_human)
                } else {
                    let n = n - monkey_map.get(a.as_str()).unwrap().evaluate(monkey_map);
                    monkey_map.get(b.as_str()).unwrap().solve(n, b, monkey_map, contains_human)
                }
            }
            Self::Subtract(a, b) => {
                if contains_human.contains(a.as_str()) {
                    let n = n + monkey_map.get(b.as_str()).unwrap().evaluate(monkey_map);
                    monkey_map.get(a.as_str()).unwrap().solve(n, a, monkey_map, contains_human)
                } else {
                    let n = -(n - monkey_map.get(a.as_str()).unwrap().evaluate(monkey_map));
                    monkey_map.get(b.as_str()).unwrap().solve(n, b, monkey_map, contains_human)
                }
            }
            Self::Multiply(a, b) => {
                if contains_human.contains(a.as_str()) {
                    let n = n / monkey_map.get(b.as_str()).unwrap().evaluate(monkey_map);
                    monkey_map.get(a.as_str()).unwrap().solve(n, a, monkey_map, contains_human)
                } else {
                    let n = n / monkey_map.get(a.as_str()).unwrap().evaluate(monkey_map);
                    monkey_map.get(b.as_str()).unwrap().solve(n, b, monkey_map, contains_human)
                }
            }
            Self::Divide(a, b) => {
                if contains_human.contains(a.as_str()) {
                    let n = n * monkey_map.get(b.as_str()).unwrap().evaluate(monkey_map);
                    monkey_map.get(a.as_str()).unwrap().solve(n, a, monkey_map, contains_human)
                } else {
                    let n = monkey_map.get(a.as_str()).unwrap().evaluate(monkey_map) / n;
                    monkey_map.get(b.as_str()).unwrap().solve(n, b, monkey_map, contains_human)
                }
            }
        }
    }
}

fn solve(input: &str) -> i64 {
    let monkeys = parse_input(input);

    let mut monkey_map: HashMap<&str, &Monkey> = HashMap::with_capacity(monkeys.len());
    for (name, monkey) in &monkeys {
        monkey_map.insert(name, monkey);
    }

    monkey_map.get("root").unwrap().evaluate(&monkey_map)
}

fn solve_part_2(input: &str) -> i64 {
    let monkeys = parse_input(input);

    let mut monkey_map: HashMap<&str, &Monkey> = HashMap::with_capacity(monkeys.len());
    for (name, monkey) in &monkeys {
        monkey_map.insert(name, monkey);
    }

    let mut contains_human: HashSet<&str> = HashSet::new();
    contains_human.insert("humn");
    monkey_map.get("root").unwrap().populate_contains_human("root", &monkey_map, &mut contains_human);

    match monkey_map.get("root").unwrap() {
        Monkey::Add(a, b) => {
            if contains_human.contains(a.as_str()) {
                let b = monkey_map.get(b.as_str()).unwrap().evaluate(&monkey_map);
                monkey_map.get(a.as_str()).unwrap().solve(b, a, &monkey_map, &contains_human)
            } else {
                let a = monkey_map.get(a.as_str()).unwrap().evaluate(&monkey_map);
                monkey_map.get(b.as_str()).unwrap().solve(a, b, &monkey_map, &contains_human)
            }
        }
        _ => panic!("root is not add"),
    }
}

fn parse_input(input: &str) -> Vec<(String, Monkey)> {
    input.lines().map(|line| {
        let split: Vec<_> = line.split(' ').collect();
        let name = String::from(&split[0][..split[0].len() - 1]);

        let op = match split[1..] {
            [n] => {
                Monkey::Constant(n.parse::<i64>().expect("single word should be an integer"))
            }
            [a, "+", b] => {
                Monkey::Add(String::from(a), String::from(b))
            }
            [a, "-", b] => {
                Monkey::Subtract(String::from(a), String::from(b))
            }
            [a, "*", b] => {
                Monkey::Multiply(String::from(a), String::from(b))
            }
            [a, "/", b] => {
                Monkey::Divide(String::from(a), String::from(b))
            }
            _ => panic!("unexpected operation: {split:?}"),
        };

        (name, op)
    })
        .collect()
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