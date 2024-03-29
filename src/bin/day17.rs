//! Day 17: Pyroclastic Flow
//!
//! <https://adventofcode.com/2022/day/17>

use std::cmp;
use std::collections::HashMap;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
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

impl Tetronimo {
    fn highest_point_y(&self) -> Option<i64> {
        self.points.iter().map(|p| p.y).max()
    }
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

#[derive(Debug)]
struct TetrisChamber {
    // Stores the occupied y coordinates in each column, in sorted order
    occupied_points: Vec<Vec<i64>>,
}

impl TetrisChamber {
    fn new() -> Self {
        Self {
            occupied_points: vec![Vec::new(); CHAMBER_WIDTH as usize],
        }
    }

    fn contains(&self, p: &Point) -> bool {
        self.occupied_points[p.x as usize]
            .binary_search(&p.y)
            .is_ok()
    }

    fn insert(&mut self, p: &Point) {
        let col = &mut self.occupied_points[p.x as usize];
        let mut i = col.len();
        while i > 0 && col[i - 1] > p.y {
            i -= 1;
        }
        col.insert(i, p.y);
    }

    fn extend<'a>(&mut self, points: impl IntoIterator<Item = &'a Point>) {
        for p in points {
            self.insert(p);
        }
    }

    // Find the column with the lowest max height, then return all points at or higher than that
    // height with their heights normalized to the lowest max
    fn determine_highest_points(&self) -> Vec<Point> {
        let lowest_max = self
            .occupied_points
            .iter()
            .map(|col| col.last().copied().unwrap_or(0))
            .min()
            .unwrap();

        let mut result: Vec<_> = self
            .occupied_points
            .iter()
            .enumerate()
            .flat_map(|(x, col)| {
                let x = x as i64;
                col.iter()
                    .rev()
                    .copied()
                    .map_while(|y| {
                        if y >= lowest_max {
                            Some(Point::new(x, y - lowest_max))
                        } else {
                            None
                        }
                    })
                    .collect::<Vec<_>>()
            })
            .collect();

        result.sort();
        result
    }
}

#[derive(Debug, PartialEq, Eq, Hash)]
struct RepititionDetectionKey {
    highest_points: Vec<Point>,
    jet_stream_index: usize,
    tetronimo_type_index: usize,
}

const CHAMBER_WIDTH: i64 = 7;

const PART_1_TO_DROP: usize = 2022;

const PART_2_TO_DROP: i64 = 1_000_000_000_000;

fn solve(input: &str) -> i64 {
    let line = input.lines().next().expect("input should not be empty");
    let mut jet_iter = repeating_jet_iterator(line);

    let mut tetronimo_type_iter = repeating_tetronimo_type_iterator();

    let mut tower_height = 0;
    let mut tetris_chamber = TetrisChamber::new();

    for _ in 0..PART_1_TO_DROP {
        let mut tetronimo = tetronimo_type_iter
            .next()
            .unwrap()
            .new_tetronimo(2, tower_height + 4);

        tetronimo = drop_tetronimo(tetronimo, &tetris_chamber, &mut jet_iter);

        tetris_chamber.extend(&tetronimo.points);
        tower_height = cmp::max(tower_height, tetronimo.highest_point_y().unwrap());
    }

    tower_height
}

fn solve_part_2(input: &str) -> i64 {
    let line = input.lines().next().expect("input should not be empty");
    let mut jet_iter = repeating_jet_iterator(line).peekable();

    let mut tetronimo_type_iter = repeating_tetronimo_type_iterator();

    let mut tower_height = 0;
    let mut tetris_chamber = TetrisChamber::new();

    let mut repitition_detection_map: HashMap<RepititionDetectionKey, (usize, i64)> =
        HashMap::new();

    for i in 1.. {
        let mut tetronimo = tetronimo_type_iter
            .next()
            .unwrap()
            .new_tetronimo(2, tower_height + 4);

        tetronimo = drop_tetronimo(tetronimo, &tetris_chamber, &mut jet_iter);

        tetris_chamber.extend(&tetronimo.points);
        tower_height = cmp::max(tower_height, tetronimo.highest_point_y().unwrap());

        let highest_points = tetris_chamber.determine_highest_points();
        let &(jet_stream_index, _) = jet_iter.peek().unwrap();
        let key = RepititionDetectionKey {
            highest_points,
            jet_stream_index,
            tetronimo_type_index: i % 5,
        };
        if let Some(&(earlier_iteration, earlier_height)) = repitition_detection_map.get(&key) {
            let i = i as i64;
            let earlier_iteration = earlier_iteration as i64;

            let div = (PART_2_TO_DROP - i) / (i - earlier_iteration);
            let rem = (PART_2_TO_DROP - i) % (i - earlier_iteration);
            if rem == 0 {
                return tower_height + div * (tower_height - earlier_height);
            }
        }

        repitition_detection_map.insert(key, (i, tower_height));
    }

    panic!("no solution found");
}

fn drop_tetronimo(
    tetronimo: Tetronimo,
    tetris_chamber: &TetrisChamber,
    jet_iter: &mut impl Iterator<Item = (usize, i64)>,
) -> Tetronimo {
    let mut tetronimo = tetronimo;

    loop {
        let (_, jet_direction) = jet_iter.next().unwrap();
        tetronimo = try_move(&tetronimo, tetris_chamber, jet_direction, 0).unwrap_or(tetronimo);

        match try_move(&tetronimo, tetris_chamber, 0, -1) {
            Some(moved_tetronimo) => {
                tetronimo = moved_tetronimo;
            }
            None => break,
        }
    }

    tetronimo
}

fn try_move(
    tetronimo: &Tetronimo,
    tetris_chamber: &TetrisChamber,
    dx: i64,
    dy: i64,
) -> Option<Tetronimo> {
    let mut new_points = Vec::with_capacity(tetronimo.points.len());

    for point in &tetronimo.points {
        let new_point = Point::new(point.x + dx, point.y + dy);

        if new_point.x < 0
            || new_point.x >= CHAMBER_WIDTH
            || new_point.y <= 0
            || tetris_chamber.contains(&new_point)
        {
            return None;
        }
        new_points.push(new_point);
    }

    Some(Tetronimo { points: new_points })
}

fn repeating_jet_iterator(line: &str) -> impl Iterator<Item = (usize, i64)> + '_ {
    line.chars()
        .map(|c| match c {
            '<' => -1,
            '>' => 1,
            _ => panic!("unexpected character in jet stream: {c}"),
        })
        .enumerate()
        .cycle()
}

fn repeating_tetronimo_type_iterator() -> impl Iterator<Item = TetronimoType> {
    vec![
        TetronimoType::Minus,
        TetronimoType::Plus,
        TetronimoType::BackwardsL,
        TetronimoType::Line,
        TetronimoType::Square,
    ]
    .into_iter()
    .cycle()
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
