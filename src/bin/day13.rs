//! Day 13: Distress Signal
//!
//! <https://adventofcode.com/2022/day/13>

use std::cmp::Ordering;
use std::iter::Peekable;

#[derive(Debug, PartialEq, Eq, Clone)]
enum ListItem {
    Int(u32),
    List(Vec<ListItem>),
}

impl PartialOrd<Self> for ListItem {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for ListItem {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (Self::Int(a), Self::Int(b)) => a.cmp(b),
            (Self::List(a), Self::List(b)) => {
                for (a_item, b_item) in a.iter().zip(b) {
                    match a_item.cmp(b_item) {
                        Ordering::Equal => {}
                        ordering => {
                            return ordering;
                        }
                    }
                }

                a.len().cmp(&b.len())
            }
            (Self::Int(_), Self::List(_)) => {
                let self_as_list = Self::List(vec![self.clone()]);
                self_as_list.cmp(other)
            }
            (Self::List(_), Self::Int(_)) => other.cmp(self).reverse(),
        }
    }
}

fn solve(input: &str) -> usize {
    let input = parse_input(input);

    input
        .into_iter()
        .enumerate()
        .filter(|(_, (a, b))| a.cmp(b).is_le())
        .map(|(i, _)| i + 1)
        .sum()
}

fn solve_part_2(input: &str) -> usize {
    let input = parse_input(input);

    let mut all_items: Vec<_> = input.into_iter().flat_map(|(a, b)| vec![a, b]).collect();

    let divider_packet_2 = ListItem::List(vec![ListItem::List(vec![ListItem::Int(2)])]);
    let divider_packet_6 = ListItem::List(vec![ListItem::List(vec![ListItem::Int(6)])]);
    all_items.push(divider_packet_2.clone());
    all_items.push(divider_packet_6.clone());

    all_items.sort();

    let index2 = all_items.binary_search(&divider_packet_2).unwrap();
    let index6 = all_items.binary_search(&divider_packet_6).unwrap();
    (index2 + 1) * (index6 + 1)
}

fn parse_input(input: &str) -> Vec<(ListItem, ListItem)> {
    let lines: Vec<_> = input.lines().collect();
    lines
        .split(|s| s.is_empty())
        .map(|line_pair| {
            let [a, b] = match line_pair {
                [a, b] => [a, b],
                _ => panic!("unexpected slice size: {line_pair:?}"),
            };

            let a = parse_list_item(&mut a.chars().peekable());
            let b = parse_list_item(&mut b.chars().peekable());
            (a, b)
        })
        .collect()
}

fn parse_list_item<I>(iter: &mut Peekable<I>) -> ListItem
where
    I: Iterator<Item = char>,
{
    match iter.peek().unwrap() {
        '[' => ListItem::List(parse_list(iter)),
        _c @ '0'..='9' => parse_int(iter),
        _ => panic!("unexpected char"),
    }
}

fn parse_int<I>(iter: &mut Peekable<I>) -> ListItem
where
    I: Iterator<Item = char>,
{
    let mut s = String::new();
    while let Some(&c) = iter.peek() {
        if c == ',' || c == ']' {
            break;
        }

        s.push(iter.next().unwrap());
    }
    ListItem::Int(s.parse().expect("list item should be an integer"))
}

fn parse_list<I>(iter: &mut Peekable<I>) -> Vec<ListItem>
where
    I: Iterator<Item = char>,
{
    // Skip '['
    iter.next();

    let mut items: Vec<ListItem> = Vec::new();
    while let Some(&c) = iter.peek() {
        if c == ']' {
            iter.next();
            break;
        }

        items.push(parse_list_item(iter));

        if iter.peek() == Some(&',') {
            iter.next();
        }
    }

    items
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

    const SAMPLE_INPUT: &str = include_str!("sample_input/sample13.txt");

    #[test]
    fn test_sample_input_part_1() {
        assert_eq!(13, solve(SAMPLE_INPUT));
    }

    #[test]
    fn test_sample_input_part_2() {
        assert_eq!(140, solve_part_2(SAMPLE_INPUT));
    }
}
