//! Day 11: Monkey in the Middle
//! https://adventofcode.com/2022/day/11

enum MonkeyOperation {
    Add(u64),
    Multiply(u64),
    Square,
}

impl MonkeyOperation {
    fn from_str(s: &str) -> Self {
        let split: Vec<_> = s.split(' ').collect();
        match split.as_slice() {
            ["old", "*", "old"] => Self::Square,
            ["old", "*", operand] => {
                let operand: u64 = operand.parse().expect("operand should be an integer");
                Self::Multiply(operand)
            }
            ["old", "+", operand] => {
                let operand: u64 = operand.parse().expect("operand should be an integer");
                Self::Add(operand)
            }
            _ => panic!("unexpected operation: {s}")
        }
    }

    fn apply(&self, item: u64) -> u64 {
        match *self {
            Self::Add(operand) => item + operand,
            Self::Multiply(operand) => item * operand,
            Self::Square => item * item,
        }
    }
}

struct Monkey {
    items: Vec<u64>,
    operation: MonkeyOperation,
    divisible_test: u64,
    true_monkey: usize,
    false_monkey: usize,
}

impl Monkey {
    fn new(
        items: Vec<u64>,
        operation: MonkeyOperation,
        divisible_test: u64,
        true_monkey: usize,
        false_monkey: usize,
    ) -> Self {
        Self {
            items,
            operation,
            divisible_test,
            true_monkey,
            false_monkey,
        }
    }
}

const PART_1_ROUNDS: usize = 20;
const PART_2_ROUNDS: usize = 10000;

fn solve(input: &str, rounds: usize, divide_by_three: bool) -> u64 {
    let mut monkeys = parse_input(input);

    let test_product: u64 = monkeys.iter().map(|monkey| monkey.divisible_test).product();

    let mut inspection_counts: Vec<u64> = vec![0; monkeys.len()];
    for _ in 0..rounds {
        for i in 0..monkeys.len() {
            let mut monkey_items: Vec<u64> = Vec::with_capacity(monkeys[i].items.len());
            monkey_items.append(&mut monkeys[i].items);

            for &item in &monkey_items {
                let mut item = monkeys[i].operation.apply(item);
                if divide_by_three {
                    item /= 3;
                } else {
                    // Why this works:
                    // For any non-negative integers a and b and any positive integer n:
                    //   (a + b) mod n == ((a mod n) + b) mod n
                    //   (a * b) mod n == ((a mod n) * b) mod n
                    // For any non-negative integer a and any positive integers n and k:
                    //   (a mod kn) mod n == a mod n
                    // All monkey operations are either addition or multiplication, and the product
                    // of all of the divisors is by definition a multiple of every divisor.
                    item %= test_product;
                }

                let new_monkey = if item % monkeys[i].divisible_test == 0 {
                    monkeys[i].true_monkey
                } else {
                    monkeys[i].false_monkey
                };

                monkeys[new_monkey].items.push(item);
            }

            inspection_counts[i] += monkey_items.len() as u64;
        }
    }

    inspection_counts.sort_by(|a, b| b.cmp(a));
    inspection_counts[0] * inspection_counts[1]
}

fn solve_part_1(input: &str) -> u64 {
    solve(input, PART_1_ROUNDS, true)
}

fn solve_part_2(input: &str) -> u64 {
    solve(input, PART_2_ROUNDS, false)
}

fn parse_input(input: &str) -> Vec<Monkey> {
    let lines: Vec<_> = input.lines().collect();
    lines.split(|s| s.is_empty())
        .map(parse_monkey)
        .collect()
}

fn parse_monkey(lines: &[&str]) -> Monkey {
    let mut iter = lines.iter();

    iter.next().expect("should be monkey header line");

    let items_line = iter.next().expect("starting items line");
    let starting_items: Vec<_> = items_line["  Starting items: ".len()..].split(", ")
        .map(|item| item.parse::<u64>().expect("every starting item should be an integer"))
        .collect();

    let operation_line = iter.next().expect("operation line");
    let operation_str = &operation_line["  Operation: new = ".len()..];
    let operation = MonkeyOperation::from_str(operation_str);

    let test_line = iter.next().expect("test line");
    let divisible_test: u64 = test_line.split(' ').last().expect("Test line should have a space")
        .parse().expect("divisible by test should be an integer");

    let true_line = iter.next().expect("true line");
    let true_monkey: usize = true_line.split(' ').last().expect("If true line should have a space")
        .parse().expect("if true monkey should be an integer");
    let false_line = iter.next().expect("false line");
    let false_monkey: usize = false_line.split(' ').last().expect("If false line should have a space")
        .parse().expect("if false monkey should be an integer");

    Monkey::new(starting_items, operation, divisible_test, true_monkey, false_monkey)
}

fn main() {
    let input = advent_of_code_2022::read_input().expect("unable to read input file");

    let solution1 = solve_part_1(&input);
    println!("{solution1}");

    let solution2 = solve_part_2(&input);
    println!("{solution2}");
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_INPUT: &str = include_str!("sample_input/sample11.txt");

    #[test]
    fn test_sample_input_part_1() {
        assert_eq!(10605, solve_part_1(SAMPLE_INPUT));
    }

    #[test]
    fn test_sample_input_part_2() {
        assert_eq!(2713310158, solve_part_2(SAMPLE_INPUT));
    }
}