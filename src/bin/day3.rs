use std::collections::HashSet;

fn priority(c: char) -> u32 {
    if c.is_lowercase() {
        (c as u32) - ('a' as u32) + 1
    } else {
        (c as u32) - ('A' as u32) + 27
    }
}

fn solve(input: &str) -> u32 {
    input.lines().map(|line| {
        let (lhalf, rhalf) = line.split_at(line.len() / 2);

        let lchars: HashSet<char> = HashSet::from_iter(lhalf.chars());

        let c = rhalf.chars().find(|c| {
            lchars.contains(c)
        }).expect("there should be a character in both l and r");

        priority(c)
    })
        .sum()
}

fn solve_part_2(input: &str) -> u32 {
    let lines: Vec<&str> = input.lines().collect();
    lines.chunks(3).map(|chunk| {
        let [a, b, c] = match chunk {
            [a, b, c] => [a, b, c],
            _ => panic!("the number of lines should be a multiple of 3"),
        };

        let achars: HashSet<char> = HashSet::from_iter(a.chars());
        let bchars: HashSet<char> = HashSet::from_iter(b.chars());

        let ch = c.chars().find(|ch| {
            achars.contains(ch) && bchars.contains(ch)
        }).expect("there should a be a character in all three lines");

        priority(ch)
    })
        .sum()
}

fn main() {
    let input = advent_of_code_2022::read_input().expect("unable to read input file");

    let solution1 = solve(&input);
    println!("{solution1}");

    let solution2 = solve_part_2(&input);
    println!("{solution2}");
}