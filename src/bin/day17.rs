//! Day 17: Pyroclastic Flow
//! https://adventofcode.com/2022/day/17

use std::cmp;
use std::collections::{HashMap, HashSet};

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Point {
    x: i64,
    y: i64,
}

impl Point {
    fn new(x: i64, y: i64) -> Self {
        Self { x, y }
    }
}

#[derive(Debug, Clone)]
struct Tetronimo {
    points: Vec<Point>,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum TetronimoType {
    Minus,
    Plus,
    BackwardsL,
    Line,
    Square,
}

impl TetronimoType {
    fn new_tetronimo(&self, min_x: i64, min_y: i64) -> Tetronimo {
        let points = match self {
            Self::Minus => {
                vec![
                    Point::new(min_x, min_y),
                    Point::new(min_x + 1, min_y),
                    Point::new(min_x + 2, min_y),
                    Point::new(min_x + 3, min_y),
                ]
            }
            Self::Plus => {
                vec![
                    Point::new(min_x, min_y + 1),
                    Point::new(min_x + 1, min_y),
                    Point::new(min_x + 1, min_y + 1),
                    Point::new(min_x + 1, min_y + 2),
                    Point::new(min_x + 2, min_y + 1),
                ]
            }
            Self::BackwardsL => {
                vec![
                    Point::new(min_x, min_y),
                    Point::new(min_x + 1, min_y),
                    Point::new(min_x + 2, min_y),
                    Point::new(min_x + 2, min_y + 1),
                    Point::new(min_x + 2, min_y + 2),
                ]
            }
            Self::Line => {
                vec![
                    Point::new(min_x, min_y),
                    Point::new(min_x, min_y + 1),
                    Point::new(min_x, min_y + 2),
                    Point::new(min_x, min_y + 3),
                ]
            }
            Self::Square => {
                vec![
                    Point::new(min_x, min_y),
                    Point::new(min_x, min_y + 1),
                    Point::new(min_x + 1, min_y),
                    Point::new(min_x + 1, min_y + 1),
                ]
            }
        };
        Tetronimo { points }
    }
}

const CHAMBER_WIDTH: i64 = 7;

const PART_1_TO_DROP: usize = 2022;

fn solve(input: &str) -> i64 {
    let line = input.lines().next().expect("input should not be empty");
    let mut jet_iter = line.chars().map(|c| {
        if c == '<' {
            -1
        } else if c == '>' {
            1
        } else {
            panic!("unexpected char in jet stream: {c}");
        }
    }).cycle();

    let mut tetronimo_type_iter = vec![
        TetronimoType::Minus,
        TetronimoType::Plus,
        TetronimoType::BackwardsL,
        TetronimoType::Line,
        TetronimoType::Square,
    ].into_iter().cycle();

    let mut tower_height = 0;
    let mut occupied_points: HashSet<Point> = HashSet::new();

    for _ in 0..PART_1_TO_DROP {
        let mut tetronimo = tetronimo_type_iter.next().unwrap().new_tetronimo(2, tower_height + 3);

        loop {
            let jet_direction = jet_iter.next().unwrap();
            tetronimo = try_move(&tetronimo, &occupied_points, jet_direction, 0).unwrap_or(tetronimo);

            match try_move(&tetronimo, &occupied_points, 0, -1) {
                Some(moved_tetronimo) => {
                    tetronimo = moved_tetronimo;
                }
                None => break,
            }
        }

        occupied_points.extend(tetronimo.points.clone());
        tower_height = cmp::max(tower_height, tetronimo.points.iter().map(|p| p.y).max().unwrap() + 1);

        // println!("height is {tower_height} at iteration {i}");
        // println!("tetronimo position: {tetronimo:?}");
    }

    tower_height
}

fn solve_part_2(input: &str) -> i64 {
    let line = input.lines().next().expect("input should not be empty");
    let mut jet_iter = line.chars().map(|c| {
        if c == '<' {
            -1
        } else if c == '>' {
            1
        } else {
            panic!("unexpected char in jet stream: {c}");
        }
    }).enumerate().cycle().peekable();

    let mut tetronimo_type_iter = vec![
        TetronimoType::Minus,
        TetronimoType::Plus,
        TetronimoType::BackwardsL,
        TetronimoType::Line,
        TetronimoType::Square,
    ].into_iter().cycle();

    let mut tower_height = 0;
    let mut occupied_points: HashSet<Point> = HashSet::new();

    let mut seen_max_heights: HashMap<Vec<i64>, (i64, usize, usize)> = HashMap::new();
    seen_max_heights.insert(vec![0; 7], (0, 0, 0));

    for i in 1.. {
        let mut tetronimo = tetronimo_type_iter.next().unwrap().new_tetronimo(2, tower_height + 3);

        loop {
            let (_, jet_direction) = jet_iter.next().unwrap();
            tetronimo = try_move(&tetronimo, &occupied_points, jet_direction, 0).unwrap_or(tetronimo);

            match try_move(&tetronimo, &occupied_points, 0, -1) {
                Some(moved_tetronimo) => {
                    tetronimo = moved_tetronimo;
                }
                None => break,
            }
        }

        occupied_points.extend(tetronimo.points.clone());
        tower_height = cmp::max(tower_height, tetronimo.points.iter().map(|p| p.y).max().unwrap() + 1);

        let normalized_max_heights = normalize_max_heights(&occupied_points);
        if i % 5 == 0 {
            if let Some(&(earlier_height, earlier_iteration, earlier_jet_index)) = seen_max_heights.get(&normalized_max_heights) {
                let jet_index = jet_iter.peek().unwrap().0;
                if jet_index == earlier_jet_index {
                    let i = i as i64;
                    let earlier_iteration = earlier_iteration as i64;
                    let partial = (1000000000000 - i) / (i - earlier_iteration);
                    let rem = (1000000000000 - i) % (i - earlier_iteration);
                    if rem == 0 {
                        return tower_height + partial * (tower_height - earlier_height);
                    }
                }
            }

            seen_max_heights.insert(normalized_max_heights, (tower_height, i, jet_iter.peek().unwrap().0));
        }
    }

    0
}

fn normalize_max_heights(occupied_points: &HashSet<Point>) -> Vec<i64> {
    let mut max_per_col = vec![-1; 7];

    for point in occupied_points {
        max_per_col[point.x as usize] = cmp::max(max_per_col[point.x as usize], point.y);
    }

    let lowest_max = *max_per_col.iter().min().unwrap();

    max_per_col.into_iter().map(|i| i - lowest_max).collect()
}

fn try_move(tetronimo: &Tetronimo, occupied_points: &HashSet<Point>, dx: i64, dy: i64) -> Option<Tetronimo> {
    let mut new_points = Vec::with_capacity(tetronimo.points.len());

    for point in &tetronimo.points {
        let new_point = Point::new(point.x + dx, point.y + dy);

        if new_point.x < 0 || new_point.x >= CHAMBER_WIDTH || new_point.y < 0 || occupied_points.contains(&new_point) {
            return None;
        }
        new_points.push(new_point);
    }

    Some(Tetronimo { points: new_points })
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

    const SAMPLE_INPUT: &str = ">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>";

    #[test]
    fn test_sample_input_part_1() {
        assert_eq!(3068, solve(SAMPLE_INPUT));
    }

    #[test]
    fn test_sample_input_part_2() {
        assert_eq!(1514285714288, solve_part_2(SAMPLE_INPUT));
    }
}