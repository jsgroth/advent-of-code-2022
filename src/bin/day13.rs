//! Day 13: Distress Signal
//! https://adventofcode.com/2022/day/13

use std::cmp::Ordering;
use std::iter::Peekable;

#[derive(Debug, PartialEq, Eq, Clone)]
enum ListItem {
    Int(u32),
    List(Vec<Box<ListItem>>),
}

impl ListItem {
    fn new_list(items: Vec<ListItem>) -> Self {
        Self::List(items.into_iter().map(Box::new).collect())
    }
}

impl PartialOrd<Self> for ListItem {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for ListItem {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (ListItem::Int(a), ListItem::Int(b)) => {
                a.cmp(b)
            },
            (ListItem::List(a), ListItem::List(b)) => {
                let mut ai = 0;
                let mut bi = 0;
                while ai < a.len() && bi < b.len() {
                    match a[ai].as_ref().cmp(b[bi].as_ref()) {
                        Ordering::Equal => {},
                        ordering => {
                            return ordering;
                        }
                    }

                    ai += 1;
                    bi += 1;
                }

                if ai < a.len() {
                    Ordering::Greater
                } else if bi < b.len() {
                    Ordering::Less
                } else {
                    Ordering::Equal
                }
            }
            (ListItem::Int(a), ListItem::List(_)) => {
                let tmp = ListItem::new_list(vec![ListItem::Int(*a)]);
                tmp.cmp(other)
            }
            (ListItem::List(_), ListItem::Int(_)) => {
                other.cmp(self).reverse()
            }
        }
    }
}

fn solve(input: &str) -> usize {
    let input = parse_input(input);

    input.into_iter().enumerate().filter(|(_, (a, b))| {
        a.cmp(b) != Ordering::Greater
    })
        .map(|(i, _)| i + 1)
        .sum()
}

fn solve_part_2(input: &str) -> usize {
    let input = parse_input(input);

    let mut all_items: Vec<_> = input.into_iter().flat_map(|(a, b)| vec![a, b]).collect();

    let divider_packet_2 = divider_packet(2);
    let divider_packet_6 = divider_packet(6);
    all_items.push(divider_packet_2.clone());
    all_items.push(divider_packet_6.clone());

    all_items.sort();

    let index2 = 1 + all_items.iter().position(|item| *item == divider_packet_2).unwrap();
    let index6 = 1 + all_items.iter().position(|item| *item == divider_packet_6).unwrap();
    index2 * index6
}

fn divider_packet(value: u32) -> ListItem {
    ListItem::new_list(vec![
        ListItem::new_list(vec![
            ListItem::Int(value)
        ])
    ])
}

fn parse_input(input: &str) -> Vec<(ListItem, ListItem)> {
    let lines: Vec<_> = input.lines().collect();
    lines.split(|s| s.is_empty())
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

fn parse_list<I>(iter: &mut Peekable<I>) -> Vec<ListItem>
where I: Iterator<Item = char>
{
    // Skip '['
    iter.next();

    let mut items: Vec<ListItem> = Vec::new();
    while let Some(&c) = iter.peek() {
        if c == ']' {
            iter.next();
            break;
        }

        if c == '[' {
            items.push(parse_list_item(iter));
        } else {
            items.push(parse_int_item(iter));
        }

        if iter.peek() == Some(&',') {
            iter.next();
        }
    }

    items
}

fn parse_int_item<I>(iter: &mut Peekable<I>) -> ListItem
where I: Iterator<Item = char>
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

fn parse_list_item<I>(iter: &mut Peekable<I>) -> ListItem
where I: Iterator<Item = char>
{
    match iter.peek().unwrap() {
        '[' => ListItem::new_list(parse_list(iter)),
        _c @ '1'..='9' => parse_int_item(iter),
        _ => panic!("unexpected char"),
    }
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