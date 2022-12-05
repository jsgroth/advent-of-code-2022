//! Day 5: Supply Stacks
//! https://adventofcode.com/2022/day/5

#[derive(Debug)]
struct Move {
    from: usize,
    to: usize,
    num: usize,
}

fn solve(input: &str, can_move_in_bulk: bool) -> String {
    let (mut stacks, moves) = parse_input(input);

    for mov in &moves {
        let from_stack = &mut stacks[mov.from - 1];
        let truncated_len = from_stack.len() - mov.num;

        let mut moved_chars: Vec<char> = from_stack[truncated_len..].iter().map(|&c| c).collect();
        if !can_move_in_bulk {
            moved_chars.reverse();
        }

        from_stack.truncate(truncated_len);

        stacks[mov.to - 1].append(&mut moved_chars);
    }

    stacks.iter().map(|stack| stack.last().expect("no stack should be empty"))
        .collect()
}

fn parse_input(input: &str) -> (Vec<Vec<char>>, Vec<Move>) {
    let lines: Vec<&str> = input.lines().collect();

    let empty_line_index = lines.iter().position(|line| line.is_empty())
        .expect("there should be an empty line");

    let stacks = parse_stacks(&lines[..empty_line_index]);

    let moves = parse_moves(&lines[empty_line_index+1..]);

    (stacks, moves)
}

fn parse_stacks(lines: &[&str]) -> Vec<Vec<char>> {
    let num_stacks: usize = lines.last()
        .and_then(|line| line.split_whitespace().last())
        .and_then(|last_char| last_char.parse().ok())
        .expect("line before empty line should end in a number");

    let mut stacks: Vec<Vec<char>> = Vec::with_capacity(num_stacks);
    for _ in 0..num_stacks {
        stacks.push(Vec::new());
    }

    for line in lines[..lines.len()-1].iter().rev() {
        let line = line.as_bytes();
        for i in 0..num_stacks {
            let j = 4 * i + 1;
            if let Some(&c) = line.get(j) {
                let c = c as char;
                if c != ' ' {
                    stacks[i].push(c);
                }
            }
        }
    }

    stacks
}

fn parse_moves(lines: &[&str]) -> Vec<Move> {
    let mut moves: Vec<Move> = Vec::with_capacity(lines.len());

    for line in lines {
        let mut split = line.split_whitespace();

        assert_eq!(split.next(), Some("move"));
        let num: usize = split.next().unwrap().parse().unwrap();

        assert_eq!(split.next(), Some("from"));
        let from: usize = split.next().unwrap().parse().unwrap();

        assert_eq!(split.next(), Some("to"));
        let to: usize = split.next().unwrap().parse().unwrap();

        moves.push(Move { from, to, num });
    }

    moves
}

fn main() {
    let input = advent_of_code_2022::read_input().expect("unable to read input file");

    let solution1 = solve(&input, false);
    println!("{solution1}");

    let solution2 = solve(&input, true);
    println!("{solution2}");
}