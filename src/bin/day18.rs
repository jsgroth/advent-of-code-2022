//! Day 18: Boiling Boulders
//!
//! <https://adventofcode.com/2022/day/18>

use std::cmp;
use std::collections::{HashSet, VecDeque};

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Cube {
    x: i32,
    y: i32,
    z: i32,
}

impl Cube {
    fn new(x: i32, y: i32, z: i32) -> Self {
        Self { x, y, z }
    }
}

#[derive(Debug)]
struct Shifted3dGrid<T> {
    grid: Vec<Vec<Vec<T>>>,
    x_shift: i32,
    y_shift: i32,
    z_shift: i32,
}

impl<T: Copy + Default> Shifted3dGrid<T> {
    fn new(min_x: i32, max_x: i32, min_y: i32, max_y: i32, min_z: i32, max_z: i32) -> Self {
        let x_size = (max_x - min_x + 3) as usize;
        let y_size = (max_y - min_y + 3) as usize;
        let z_size = (max_z - min_z + 3) as usize;

        let grid = vec![vec![vec![Default::default(); z_size]; y_size]; x_size];

        Self {
            grid,
            x_shift: -min_x + 1,
            y_shift: -min_y + 1,
            z_shift: -min_z + 1,
        }
    }

    fn get(&self, x: i32, y: i32, z: i32) -> Option<T> {
        let x = x + self.x_shift;
        let y = y + self.y_shift;
        let z = z + self.z_shift;

        if x >= 0
            && y >= 0
            && z >= 0
            && x < self.grid.len() as i32
            && y < self.grid[0].len() as i32
            && z < self.grid[0][0].len() as i32
        {
            Some(self.grid[x as usize][y as usize][z as usize])
        } else {
            None
        }
    }

    fn set(&mut self, x: i32, y: i32, z: i32, value: T) {
        let x = (x + self.x_shift) as usize;
        let y = (y + self.y_shift) as usize;
        let z = (z + self.z_shift) as usize;
        self.grid[x][y][z] = value;
    }
}

const DIRECTIONS: [(i32, i32, i32); 6] = [
    (1, 0, 0),
    (-1, 0, 0),
    (0, 1, 0),
    (0, -1, 0),
    (0, 0, 1),
    (0, 0, -1),
];

fn solve(input: &str) -> usize {
    let cubes = parse_input(input);

    let cubes_set: HashSet<_> = cubes.iter().copied().collect();

    cubes
        .iter()
        .map(|cube| {
            DIRECTIONS
                .iter()
                .filter(|&(dx, dy, dz)| {
                    !cubes_set.contains(&Cube::new(cube.x + dx, cube.y + dy, cube.z + dz))
                })
                .count()
        })
        .sum()
}

fn solve_part_2(input: &str) -> usize {
    let cubes = parse_input(input);

    let cubes_set: HashSet<_> = cubes.iter().copied().collect();

    let (min_x, min_y, min_z) = get_minimums(&cubes);
    let (max_x, max_y, max_z) = get_maximums(&cubes);

    let mut water_grid = Shifted3dGrid::new(min_x, max_x, min_y, max_y, min_z, max_z);
    floodfill_3d(
        &mut water_grid,
        &cubes_set,
        Cube::new(min_x - 1, min_y - 1, min_z - 1),
    );

    cubes
        .iter()
        .map(|cube| {
            DIRECTIONS
                .iter()
                .filter(|&(dx, dy, dz)| {
                    water_grid.get(cube.x + dx, cube.y + dy, cube.z + dz) == Some(true)
                })
                .count()
        })
        .sum()
}

fn floodfill_3d(shifted_grid: &mut Shifted3dGrid<bool>, cubes: &HashSet<Cube>, start: Cube) {
    let mut queue: VecDeque<Cube> = VecDeque::new();
    shifted_grid.set(start.x, start.y, start.z, true);
    queue.push_back(start);

    while !queue.is_empty() {
        let cube = queue.pop_front().unwrap();
        for (dx, dy, dz) in DIRECTIONS {
            let new_cube = Cube::new(cube.x + dx, cube.y + dy, cube.z + dz);
            if !cubes.contains(&new_cube)
                && shifted_grid.get(new_cube.x, new_cube.y, new_cube.z) == Some(false)
            {
                shifted_grid.set(new_cube.x, new_cube.y, new_cube.z, true);
                queue.push_back(new_cube);
            }
        }
    }
}

fn get_minimums(cubes: &[Cube]) -> (i32, i32, i32) {
    cubes.iter().fold(
        (i32::MAX, i32::MAX, i32::MAX),
        |(min_x, min_y, min_z), cube| {
            (
                cmp::min(min_x, cube.x),
                cmp::min(min_y, cube.y),
                cmp::min(min_z, cube.z),
            )
        },
    )
}

fn get_maximums(cubes: &[Cube]) -> (i32, i32, i32) {
    cubes.iter().fold(
        (i32::MIN, i32::MIN, i32::MIN),
        |(max_x, max_y, max_z), cube| {
            (
                cmp::max(max_x, cube.x),
                cmp::max(max_y, cube.y),
                cmp::max(max_z, cube.z),
            )
        },
    )
}

fn parse_input(input: &str) -> Vec<Cube> {
    input
        .lines()
        .map(|line| {
            let split: Vec<_> = line
                .split(',')
                .map(|s| s.parse::<i32>().expect("coordinate should be an integer"))
                .collect();
            Cube::new(split[0], split[1], split[2])
        })
        .collect()
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

    const SAMPLE_INPUT: &str = include_str!("sample_input/sample18.txt");

    #[test]
    fn test_sample_input_part_1() {
        assert_eq!(64, solve(SAMPLE_INPUT));
    }

    #[test]
    fn test_sample_input_part_2() {
        assert_eq!(58, solve_part_2(SAMPLE_INPUT));
    }
}
