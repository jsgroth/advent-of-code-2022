//! Day 19: Not Enough Minerals
//! https://adventofcode.com/2022/day/

use std::cmp;
use std::collections::HashMap;

#[derive(Debug)]
struct Blueprint {
    ore_robot_ore_cost: u32,
    clay_robot_ore_cost: u32,
    obsidian_robot_ore_cost: u32,
    obsidian_robot_clay_cost: u32,
    geode_robot_ore_cost: u32,
    geode_robot_obsidian_cost: u32,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct Key {
    ore: u32,
    ore_robots: u32,
    clay: u32,
    clay_robots: u32,
    obsidian: u32,
    obsidian_robots: u32,
    remaining: u32,
}

fn solve(input: &str) -> u32 {
    let blueprints = parse_input(input);

    blueprints.into_iter().enumerate().map(|(i, blueprint)| {
        println!("iteration {i}");
        let result = find_max_for_blueprint(&blueprint, 24) * ((i + 1) as u32);
        println!("{result}");
        result
    })
        .sum()
}

fn solve_part_2(input: &str) -> u32 {
    let blueprints = parse_input(input);

    blueprints[..cmp::min(3, blueprints.len())].into_iter().enumerate().map(|(i, blueprint)| {
        println!("iteration {i}");
        let result = find_max_for_blueprint(&blueprint, 32);
        println!("{result}");
        result
    })
        .product()
}

fn find_max_for_blueprint(blueprint: &Blueprint, remaining: u32) -> u32 {
    search(blueprint, Key {
        ore: 0,
        ore_robots: 1,
        clay: 0,
        clay_robots: 0,
        obsidian: 0,
        obsidian_robots: 0,
        remaining,
    }, &mut HashMap::new())
}

fn search(
    blueprint: &Blueprint,
    key: Key,
    cache: &mut HashMap<Key, u32>,
) -> u32 {
    if key.remaining == 0 {
        return 0;
    }

    if let Some(&value) = cache.get(&key) {
        return value;
    }

    let mut future_result = u32::MIN;

    if key.ore >= blueprint.geode_robot_ore_cost && key.obsidian >= blueprint.geode_robot_obsidian_cost {
        future_result = key.remaining - 1 + cmp::max(future_result, search(blueprint, Key {
            ore: key.ore - blueprint.geode_robot_ore_cost + key.ore_robots,
            clay: key.clay + key.clay_robots,
            obsidian: key.obsidian - blueprint.geode_robot_obsidian_cost + key.obsidian_robots,
            remaining: key.remaining - 1,
            ..key
        }, cache));
    } else {
        if key.ore >= blueprint.ore_robot_ore_cost {
            future_result = cmp::max(future_result, search(blueprint, Key {
                ore: key.ore - blueprint.ore_robot_ore_cost + key.ore_robots,
                ore_robots: key.ore_robots + 1,
                clay: key.clay + key.clay_robots,
                obsidian: key.obsidian + key.obsidian_robots,
                remaining: key.remaining - 1,
                ..key
            }, cache));
        }

        if key.ore >= blueprint.clay_robot_ore_cost {
            future_result = cmp::max(future_result, search(blueprint, Key {
                ore: key.ore - blueprint.clay_robot_ore_cost + key.ore_robots,
                clay: key.clay + key.clay_robots,
                clay_robots: key.clay_robots + 1,
                obsidian: key.obsidian + key.obsidian_robots,
                remaining: key.remaining - 1,
                ..key
            }, cache))
        }

        if key.ore >= blueprint.obsidian_robot_ore_cost && key.clay >= blueprint.obsidian_robot_clay_cost {
            future_result = cmp::max(future_result, search(blueprint, Key {
                ore: key.ore - blueprint.obsidian_robot_ore_cost + key.ore_robots,
                clay: key.clay - blueprint.obsidian_robot_clay_cost + key.clay_robots,
                obsidian: key.obsidian + key.obsidian_robots,
                obsidian_robots: key.obsidian_robots + 1,
                remaining: key.remaining - 1,
                ..key
            }, cache));
        }

        future_result = cmp::max(future_result, search(blueprint, Key {
            ore: key.ore + key.ore_robots,
            clay: key.clay + key.clay_robots,
            obsidian: key.obsidian + key.obsidian_robots,
            remaining: key.remaining - 1,
            ..key
        }, cache));
    }

    let result = future_result;
    cache.insert(key, result);
    result
}

fn parse_input(input: &str) -> Vec<Blueprint> {
    input.lines().map(|line| {
        let mut split = line.split(' ').skip(6);

        let ore_robot_ore_cost = split.next().unwrap().parse().expect("ore robot ore cost");

        let mut split = split.skip(5);
        let clay_robot_ore_cost = split.next().unwrap().parse().expect("clay robot ore cost");

        let mut split = split.skip(5);
        let obsidian_robot_ore_cost = split.next().unwrap().parse().expect("obsidian robot ore cost");

        let mut split = split.skip(2);
        let obsidian_robot_clay_cost = split.next().unwrap().parse().expect("obsidian robot clay cost");

        let mut split = split.skip(5);
        let geode_robot_ore_cost = split.next().unwrap().parse().expect("geode robot ore cost");

        let mut split = split.skip(2);
        let geode_robot_obsidian_cost = split.next().unwrap().parse().expect("geode robot obsidian cost");

        Blueprint {
            ore_robot_ore_cost,
            clay_robot_ore_cost,
            obsidian_robot_ore_cost,
            obsidian_robot_clay_cost,
            geode_robot_ore_cost,
            geode_robot_obsidian_cost,
        }
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

    const SAMPLE_INPUT: &str = include_str!("sample_input/sample19.txt");

    #[test]
    fn test_sample_input_part_1() {
        assert_eq!(33, solve(SAMPLE_INPUT));
    }

    #[test]
    fn test_sample_input_part_2() {
        assert_eq!(0, solve_part_2(SAMPLE_INPUT));
    }
}