//! Day 25: Full of Hot Air
//! https://adventofcode.com/2022/day/25

fn solve(input: &str) -> String {
    let decimal_sum: i64 = input.lines().map(parse_snafu_number).sum();

    to_snafu_number(decimal_sum)
}

fn parse_snafu_number(line: &str) -> i64 {
    line.chars()
        .rev()
        .enumerate()
        .map(|(i, c)| {
            let digit: i64 = match c {
                '=' => -2,
                '-' => -1,
                _ => (c as i64) - ('0' as i64),
            };
            digit * 5_i64.pow(i as u32)
        })
        .sum()
}

fn to_snafu_number(mut n: i64) -> String {
    let mut s = String::new();

    while n > 0 {
        let rem = (n % 5) as u8;

        match rem {
            3 => {
                s.push('=');
                n = (n + 2) / 5;
            }
            4 => {
                s.push('-');
                n = (n + 1) / 5;
            }
            _ => {
                s.push((rem + b'0') as char);
                n /= 5;
            }
        }
    }

    s.chars().rev().collect()
}

fn main() {
    let input = advent_of_code_2022::read_input().expect("unable to read input file");

    let solution = solve(&input);
    println!("{solution}");
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_INPUT: &str = include_str!("sample_input/sample25.txt");

    #[test]
    fn test_sample_input() {
        assert_eq!(String::from("2=-1=0"), solve(SAMPLE_INPUT));
    }
}
