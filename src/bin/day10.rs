//! Day 10: Cathode-Ray Tube
//! https://adventofcode.com/2022/day/10

enum Instruction {
    Noop,
    Addx(i32),
}

impl Instruction {
    fn from_line(line: &str) -> Self {
        if line == "noop" {
            Self::Noop
        } else {
            let (_, addx_operand) = line.split_once(' ').expect("every addx line should have an operand");
            let addx_operand: i32 = addx_operand.parse().expect("every addx operand should be an integer");
            Self::Addx(addx_operand)
        }
    }
}

const INITIAL_REGISTER_VALUE: i32 = 1;

const HORIZONTAL_RESOLUTION: usize = 40;

fn solve(input: &str) -> i32 {
    generate_values_iter(input)
        .enumerate()
        .skip(19)
        .step_by(40)
        .fold(0, |acc, (i, x)| {
            acc + x * ((i as i32) + 1)
        })
}

fn solve_part_2(input: &str) {
    for (i, x) in generate_values_iter(input).enumerate() {
        let j = i % HORIZONTAL_RESOLUTION;
        if j == 0 {
            println!();
        }

        if ((j as i32) - x).abs() <= 1 {
            print!("#");
        } else {
            // Print space instead of . because it makes the output more readable
            print!(" ");
        }
    }
    println!();
}

// Returns an iterator over the value of X at each cycle
fn generate_values_iter(input: &str) -> impl Iterator<Item = i32> + '_ {
    input.lines()
        .map(Instruction::from_line)
        .scan(INITIAL_REGISTER_VALUE, |x, instruction| {
            match instruction {
                Instruction::Noop => Some(vec![*x]),
                Instruction::Addx(operand) => {
                    let result = vec![*x; 2];
                    *x += operand;
                    Some(result)
                }
            }
        })
        .flatten()
}

fn main() {
    let input = advent_of_code_2022::read_input().expect("unable to read input file");

    let solution1 = solve(&input);
    println!("{solution1}");

    solve_part_2(&input);
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_INPUT: &str = "\
addx 15
addx -11
addx 6
addx -3
addx 5
addx -1
addx -8
addx 13
addx 4
noop
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx -35
addx 1
addx 24
addx -19
addx 1
addx 16
addx -11
noop
noop
addx 21
addx -15
noop
noop
addx -3
addx 9
addx 1
addx -3
addx 8
addx 1
addx 5
noop
noop
noop
noop
noop
addx -36
noop
addx 1
addx 7
noop
noop
noop
addx 2
addx 6
noop
noop
noop
noop
noop
addx 1
noop
noop
addx 7
addx 1
noop
addx -13
addx 13
addx 7
noop
addx 1
addx -33
noop
noop
noop
addx 2
noop
noop
noop
addx 8
noop
addx -1
addx 2
addx 1
noop
addx 17
addx -9
addx 1
addx 1
addx -3
addx 11
noop
noop
addx 1
noop
addx 1
noop
noop
addx -13
addx -19
addx 1
addx 3
addx 26
addx -30
addx 12
addx -1
addx 3
addx 1
noop
noop
noop
addx -9
addx 18
addx 1
addx 2
noop
noop
addx 9
noop
noop
noop
addx -1
addx 2
addx -37
addx 1
addx 3
noop
addx 15
addx -21
addx 22
addx -6
addx 1
noop
addx 2
addx 1
noop
addx -10
noop
noop
addx 20
addx 1
addx 2
addx 2
addx -6
addx -11
noop
noop
noop
";

    #[test]
    fn test_sample_input_part_1() {
        assert_eq!(13140, solve(SAMPLE_INPUT));
    }
}