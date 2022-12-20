//! Day 20: Grove Positioning System
//! https://adventofcode.com/2022/day/20

#[derive(Debug)]
struct Node {
    prev: Option<usize>,
    next: Option<usize>,
    value: i64,
}

struct NumberList {
    nodes: Vec<Node>,
    indices: Vec<usize>,
    head: usize,
    tail: usize,
}

impl NumberList {
    fn from_vec(v: Vec<i64>) -> Self {
        let mut nodes = Vec::with_capacity(v.len());
        let mut indices = Vec::with_capacity(v.len());
        for i in 0..v.len() {
            let prev = if i == 0 { None } else { Some(i - 1) };
            let next = if i == v.len() - 1 { None } else { Some(i + 1) };
            let node = Node {
                prev,
                next,
                value: v[i],
            };

            nodes.push(node);
            indices.push(i);
        }
        Self {
            nodes,
            indices,
            head: 0,
            tail: v.len() - 1,
        }
    }

    fn move_num(&mut self, index: usize) {
        let n = self.nodes[index].value;

        if n > 0 {
            for _ in 0..(n % (self.nodes.len() - 1) as i64) {
                match self.nodes[index].next {
                    Some(next) => {
                        self.nodes[index].next = self.nodes[next].next;
                        self.nodes[next].prev = self.nodes[index].prev;

                        if let Some(prev) = self.nodes[index].prev {
                            self.nodes[prev].next = Some(next);
                        }
                        if let Some(next_next) = self.nodes[next].next {
                            self.nodes[next_next].prev = Some(index);
                        }

                        self.nodes[index].prev = Some(next);
                        self.nodes[next].next = Some(index);

                        if self.nodes[index].next == None {
                            self.tail = index;
                        } else if self.nodes[next].prev == None {
                            self.head = next;
                        }
                    }
                    None => {
                        let prev = self.nodes[index].prev.unwrap();
                        self.nodes[prev].next = None;
                        self.tail = prev;

                        let after_head = self.nodes[self.head].next.unwrap();
                        self.nodes[index].next = Some(after_head);
                        self.nodes[after_head].prev = Some(index);
                        self.nodes[index].prev = Some(self.head);
                        self.nodes[self.head].next = Some(index);
                    }
                }
            }
        } else if n < 0 {
            for _ in 0..(n.abs() % (self.nodes.len() - 1) as i64) {
                match self.nodes[index].prev {
                    Some(prev) => {
                        self.nodes[index].prev = self.nodes[prev].prev;
                        self.nodes[prev].next = self.nodes[index].next;

                        if let Some(next) = self.nodes[index].next {
                            self.nodes[next].prev = Some(prev);
                        }
                        if let Some(prev_prev) = self.nodes[prev].prev {
                            self.nodes[prev_prev].next = Some(index);
                        }

                        self.nodes[index].next = Some(prev);
                        self.nodes[prev].prev = Some(index);

                        if self.nodes[index].prev == None {
                            self.head = prev;
                            self.nodes[prev].prev = None;

                            self.nodes[self.tail].next = Some(index);
                            self.nodes[index].prev = Some(self.tail);
                            self.nodes[index].next = None;
                            self.tail = index;
                        } else if self.nodes[prev].next == None {
                            self.tail = prev;
                        }
                    }
                    None => {
                        let next = self.nodes[index].next.unwrap();
                        self.nodes[next].prev = None;
                        self.head = next;

                        let before_tail = self.nodes[self.tail].prev.unwrap();
                        self.nodes[index].prev = Some(before_tail);
                        self.nodes[before_tail].next = Some(index);
                        self.nodes[index].next = Some(self.tail);
                        self.nodes[self.tail].prev = Some(index);
                    }
                }
            }
        }
    }

    fn get(&self, i: usize) -> i64 {
        let mut zero_node = &self.nodes[self.head];
        while zero_node.value != 0 {
            zero_node = &self.nodes[zero_node.next.unwrap()];
        }

        let i = i % self.nodes.len();
        let mut node = zero_node;
        for _ in 0..i {
            match node.next {
                Some(next) => {
                    node = &self.nodes[next];
                }
                None => {
                    node = &self.nodes[self.head];
                }
            }
        }
        node.value
    }

    fn _print(&self) {
        // for node in &self.nodes {
        //     println!("{node:?}");
        // }
        // println!("{}, {}", self.head, self.tail);
        let mut node = &self.nodes[self.head];
        let mut numbers: Vec<i64> = Vec::with_capacity(self.nodes.len());
        loop {
            numbers.push(node.value);
            match node.next {
                Some(next) => {
                    node = &self.nodes[next];
                }
                None => break,
            }
        }
        println!("{numbers:?}");
    }
}

fn solve(input: &str) -> i64 {
    let numbers = parse_input(input);

    let mut list = NumberList::from_vec(numbers);
    for i in 0..list.nodes.len() {
        list.move_num(i);
    }

    list.get(1000) + list.get(2000) + list.get(3000)
}

fn solve_part_2(input: &str) -> i64 {
    let numbers: Vec<_> = parse_input(input).into_iter().map(|n| {
        n * 811589153
    }).collect();
    let mut list = NumberList::from_vec(numbers);

    for _ in 0..10 {
        for i in 0..list.nodes.len() {
            list.move_num(i);
        }
    }

    list.get(1000) + list.get(2000) + list.get(3000)
}

fn parse_input(input: &str) -> Vec<i64> {
    input.lines()
        .map(|line| line.parse::<i64>().expect("each line should contain an integer"))
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

    const SAMPLE_INPUT: &str = include_str!("sample_input/sample20.txt");

    #[test]
    fn test_sample_input_part_1() {
        assert_eq!(3, solve(SAMPLE_INPUT));
    }

    #[test]
    fn test_sample_input_part_2() {
        assert_eq!(1623178306, solve_part_2(SAMPLE_INPUT));
    }
}