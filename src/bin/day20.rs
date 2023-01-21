//! Day 20: Grove Positioning System
//! https://adventofcode.com/2022/day/20

struct Node {
    value: i64,
    original_index: usize,
}

struct NumberList {
    nodes: Vec<Node>,
    indices: Vec<usize>,
    zero_index: usize,
}

impl NumberList {
    fn from_vec(v: Vec<i64>) -> Self {
        let mut nodes = Vec::with_capacity(v.len());
        let mut indices = Vec::with_capacity(v.len());
        let mut zero_index: Option<usize> = None;
        for (i, n) in v.into_iter().enumerate() {
            nodes.push(Node {
                value: n,
                original_index: i,
            });
            indices.push(i);
            if n == 0 {
                zero_index = Some(i);
            }
        }
        Self {
            nodes,
            indices,
            zero_index: zero_index.expect("list should contain 0"),
        }
    }

    fn move_num(&mut self, i: usize) {
        let mut index = self.indices[i];
        let n = self.nodes[index].value;

        let num_moves = n.abs() % (self.nodes.len() - 1) as i64;
        for _ in 0..num_moves {
            if index == 0 && n < 0 {
                self.swap(0, self.nodes.len() - 1);
                index = self.nodes.len() - 1;
            } else if index == self.nodes.len() - 1 && n > 0 {
                self.swap(0, self.nodes.len() - 1);
                index = 0;
            } else {
                let new_index = (index as i64 + n.signum()) as usize;
                self.swap(index, new_index);
                index = new_index;
            }
        }
    }

    fn get(&self, i: usize) -> i64 {
        let target_index = (self.indices[self.zero_index] + i) % self.nodes.len();
        self.nodes[target_index].value
    }

    fn swap(&mut self, i: usize, j: usize) {
        self.nodes.swap(i, j);
        self.indices
            .swap(self.nodes[i].original_index, self.nodes[j].original_index);
    }
}

fn solve(input: &str, multiplier: i64, iterations: usize) -> i64 {
    let numbers: Vec<_> = parse_input(input)
        .into_iter()
        .map(|n| n * multiplier)
        .collect();
    let mut list = NumberList::from_vec(numbers);

    for _ in 0..iterations {
        for i in 0..list.nodes.len() {
            list.move_num(i);
        }
    }

    list.get(1000) + list.get(2000) + list.get(3000)
}

fn solve_part_1(input: &str) -> i64 {
    solve(input, 1, 1)
}

fn solve_part_2(input: &str) -> i64 {
    solve(input, 811589153, 10)
}

fn parse_input(input: &str) -> Vec<i64> {
    input
        .lines()
        .map(|line| {
            line.parse::<i64>()
                .expect("each line should contain an integer")
        })
        .collect()
}

fn main() {
    let input = advent_of_code_2022::read_input().expect("unable to read input file");

    let solution1 = solve_part_1(&input);
    println!("{solution1}");

    let solution2 = solve_part_2(&input);
    println!("{solution2}");
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_INPUT: &str = include_str!("sample_input/sample20.txt");

    #[test]
    fn test_sample_input_part_1() {
        assert_eq!(3, solve_part_1(SAMPLE_INPUT));
    }

    #[test]
    fn test_sample_input_part_2() {
        assert_eq!(1623178306, solve_part_2(SAMPLE_INPUT));
    }
}
