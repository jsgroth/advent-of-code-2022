//! Day 10: Cathode-Ray Tube
//!
//! <https://adventofcode.com/2022/day/10>

enum Instruction {
    Noop,
    Addx(i32),
}

impl Instruction {
    fn from_line(line: &str) -> Self {
        let split: Vec<_> = line.split(' ').collect();
        match split.as_slice() {
            ["noop"] => Self::Noop,
            ["addx", operand] => {
                let operand: i32 = operand.parse().expect("addx operand should be an integer");
                Self::Addx(operand)
            }
            _ => panic!("op should be noop or addx"),
        }
    }
}

const INITIAL_REGISTER_VALUE: i32 = 1;

const START_COUNTING_CYCLE: usize = 20;
const COUNTING_CYCLE_STEP: usize = 40;

const HORIZONTAL_RESOLUTION: usize = 40;
const VERTICAL_RESOLUTION: usize = 6;

fn solve(input: &str) -> i32 {
    generate_values_iter(input)
        .enumerate()
        .skip(START_COUNTING_CYCLE - 1)
        .step_by(COUNTING_CYCLE_STEP)
        .fold(0, |acc, (i, x)| acc + x * ((i as i32) + 1))
}

fn solve_part_2(input: &str) -> String {
    let mut result = String::with_capacity(VERTICAL_RESOLUTION * (HORIZONTAL_RESOLUTION + 1));

    for (i, x) in generate_values_iter(input).enumerate() {
        let j = i % HORIZONTAL_RESOLUTION;
        if j == 0 && i > 0 {
            result.push('\n');
        }

        if ((j as i32) - x).abs() <= 1 {
            result.push('#');
        } else {
            // Print space instead of . because it makes the output more readable
            result.push(' ');
        }
    }

    result
}

// Returns an iterator over the value of X at each cycle
fn generate_values_iter(input: &str) -> impl Iterator<Item = i32> + '_ {
    input
        .lines()
        .map(Instruction::from_line)
        .scan(INITIAL_REGISTER_VALUE, |x, instruction| match instruction {
            Instruction::Noop => Some(vec![*x]),
            Instruction::Addx(operand) => {
                let result = vec![*x; 2];
                *x += operand;
                Some(result)
            }
        })
        .flatten()
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

    const SAMPLE_INPUT: &str = include_str!("sample_input/sample10.txt");

    #[test]
    fn test_sample_input_part_1() {
        assert_eq!(13140, solve(SAMPLE_INPUT));
    }
}
