use std::cmp::max;

fn solve(input: &Vec<String>) -> i32 {
    let mut current_total = 0;
    let mut max_total = 0;

    for line in input {
        if line.is_empty() {
            max_total = max(current_total, max_total);
            current_total = 0;
        } else {
            let value: i32 = line.parse().expect("line should be an integer");
            current_total += value;
        }
    }

    max(current_total, max_total)
}

fn solve_part_2(input: &Vec<String>) -> i32 {
    let mut current_total = 0;
    let mut max_totals = [0, 0, 0];

    for line in input {
        if line.is_empty() {
            if current_total > max_totals[0] {
                max_totals[0] = current_total;
                max_totals.sort();
            }
            current_total = 0;
        } else {
            let value: i32 = line.parse().expect("line should be an integer");
            current_total += value;
        }
    }

    if current_total > max_totals[0] {
        max_totals[0] = current_total;
    }

    max_totals.iter().sum()
}

fn main() {
    let input = advent_of_code_2022::read_input(1).expect("unable to read input file");

    let solution1 = solve(&input);
    println!("{solution1}");

    let solution2 = solve_part_2(&input);
    println!("{solution2}");
}