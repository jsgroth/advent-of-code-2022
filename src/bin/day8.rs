//! Day 8: Treetop Tree House
//! https://adventofcode.com/2022/day/8

use std::cmp::Reverse;
use std::collections::BinaryHeap;

fn solve(input: &str) -> usize {
    let grid = parse_input(input);

    let rows = grid.len();
    let cols = grid[0].len();

    let mut visible_tree_positions = empty_matrix(rows, cols, false);

    for i in 0..rows {
        let row_right = find_visible_positions(grid[i].iter().copied());
        for j in row_right.iter().copied() {
            visible_tree_positions[i][j] = true;
        }

        let row_left = find_visible_positions(grid[i].iter().copied().rev());
        for j in row_left.iter().copied() {
            visible_tree_positions[i][cols - j - 1] = true;
        }
    }

    for j in 0..cols {
        let col_down = find_visible_positions(col_iter(&grid, j));
        for i in col_down.iter().copied() {
            visible_tree_positions[i][j] = true;
        }

        let col_up = find_visible_positions(col_iter(&grid, j).rev());
        for i in col_up.iter().copied() {
            visible_tree_positions[rows - i - 1][j] = true;
        }
    }

    visible_tree_positions.iter().flatten().filter(|b| **b).count()
}

fn solve_part_2(input: &str) -> usize {
    let grid = parse_input(input);

    let rows = grid.len();
    let cols = grid[0].len();

    (0..rows).flat_map(|i| {
        let grid_ref = &grid;
        (0..cols).map(move |j| compute_scenic_score(grid_ref, i, j))
    })
        .max()
        .unwrap()
}

fn parse_input(input: &str) -> Vec<Vec<u8>> {
    input.lines().map(|line| {
        line.as_bytes().iter().map(|c| {
            *c - ('0' as u8)
        }).collect()
    }).collect()
}

fn col_iter<'a, T>(grid: &'a Vec<Vec<T>>, j: usize) -> impl Iterator<Item = T> + DoubleEndedIterator + 'a
where T: Copy
{
    let rows = grid.len();
    (0..rows).map(move |i| grid[i][j])
}

fn empty_matrix<T: Copy>(rows: usize, cols: usize, default_value: T) -> Vec<Vec<T>> {
    (0..rows).map(|_| {
        (0..cols).map(|_| default_value).collect()
    }).collect()
}

fn find_visible_positions<I>(iter: I) -> Vec<usize>
where
    I: Iterator<Item = u8>,
{
    let mut heap: BinaryHeap<Reverse<(u8, usize)>> = BinaryHeap::new();

    for (i, height) in iter.enumerate() {
        while let Some(top) = heap.peek() {
            let (top_height, _) = top.0;
            if top_height > height {
                break;
            }

            heap.pop();
        }

        heap.push(Reverse((height, i)));
    }

    heap.iter().map(|rev| {
        let (_, i) = rev.0;
        i
    }).collect()
}

fn compute_scenic_score(grid: &Vec<Vec<u8>>, i: usize, j: usize) -> usize {
    let rows = grid.len();
    let cols = grid[0].len();

    if i == 0 || i == rows - 1 || j == 0 || j == cols - 1 {
        return 0;
    }

    let height = grid[i][j];

    let mut scenic_score = 1;

    let mut ii = i - 1;
    while ii > 0 && grid[ii][j] < height {
        ii -= 1;
    }
    scenic_score *= i - ii;

    let mut ii = i + 1;
    while ii < rows - 1 && grid[ii][j] < height {
        ii += 1;
    }
    scenic_score *= ii - i;

    let mut jj = j - 1;
    while jj > 0 && grid[i][jj] < height {
        jj -= 1;
    }
    scenic_score *= j - jj;

    let mut jj = j + 1;
    while jj < cols - 1 && grid[i][jj] < height {
        jj += 1;
    }
    scenic_score *= jj - j;

    scenic_score
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

    const SAMPLE_INPUT: &str = "\
30373
25512
65332
33549
35390
";

    #[test]
    fn test_sample_input_part_1() {
        assert_eq!(21, solve(SAMPLE_INPUT));
    }

    #[test]
    fn test_sample_input_part_2() {
        assert_eq!(8, solve_part_2(SAMPLE_INPUT));
    }
}