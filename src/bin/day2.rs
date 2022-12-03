//! Day 2: Rock Paper Scissors
//! https://adventofcode.com/2022/day/2

#[derive(PartialEq, Eq)]
enum Play {
    Rock,
    Paper,
    Scissors,
}

impl Play {
    fn from_char(c: char) -> Play {
        match c {
            'A' | 'X' => Self::Rock,
            'B' | 'Y' => Self::Paper,
            'C' | 'Z' => Self::Scissors,
            _ => panic!("{c} should be A-C or X-Z"),
        }
    }

    fn point_value(&self) -> i32 {
        match self {
            Self::Rock => 1,
            Self::Paper => 2,
            Self::Scissors => 3,
        }
    }

    fn get_winning_play(&self) -> Play {
        match self {
            Self::Rock => Self::Paper,
            Self::Paper => Self::Scissors,
            Self::Scissors => Self::Rock,
        }
    }

    fn get_losing_play(&self) -> Play {
        match self {
            Self::Rock => Self::Scissors,
            Self::Paper => Self::Rock,
            Self::Scissors => Self::Paper,
        }
    }

    fn beats(&self, other: &Play) -> bool {
        *other == self.get_losing_play()
    }
}

const WIN_SCORE: i32 = 6;
const DRAW_SCORE: i32 = 3;
const LOSE_SCORE: i32 = 0;

fn solve(input: &str) -> i32 {
    input.lines().map(|line| {
        let opponent_play = Play::from_char(line.chars().next().expect("no line should be empty"));
        let your_play = Play::from_char(line.chars().last().expect("no line should be empty"));

        let game_score = if opponent_play == your_play {
            DRAW_SCORE
        } else if your_play.beats(&opponent_play) {
            WIN_SCORE
        } else {
            LOSE_SCORE
        };

        game_score + your_play.point_value()
    })
        .sum()
}

fn solve_part_2(input: &str) -> i32 {
    input.lines().map(|line| {
        let opponent_play = Play::from_char(line.chars().next().expect("no line should be empty"));

        let last_char = line.chars().last().expect("no line should be empty");
        match last_char {
            'X' => LOSE_SCORE + opponent_play.get_losing_play().point_value(),
            'Y' => DRAW_SCORE + opponent_play.point_value(),
            'Z' => WIN_SCORE + opponent_play.get_winning_play().point_value(),
            _ => panic!("every line should end in X/Y/Z"),
        }
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