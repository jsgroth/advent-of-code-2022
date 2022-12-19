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

impl Blueprint {
    fn max_ore_cost(&self) -> u32 {
        [self.ore_robot_ore_cost, self.clay_robot_ore_cost, self.obsidian_robot_ore_cost, self.geode_robot_ore_cost].into_iter().max().unwrap()
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct SearchState {
    ore: u32,
    ore_robots: u32,
    clay: u32,
    clay_robots: u32,
    obsidian: u32,
    obsidian_robots: u32,
    remaining: u32,
}

impl SearchState {
    fn new_initial_state(remaining: u32) -> Self {
        Self {
            ore: 0,
            ore_robots: 1,
            clay: 0,
            clay_robots: 0,
            obsidian: 0,
            obsidian_robots: 0,
            remaining,
        }
    }
}

fn solve(input: &str) -> u32 {
    let blueprints = parse_input(input);

    blueprints.into_iter().enumerate().map(|(i, blueprint)| {
        let result = find_max_for_blueprint(&blueprint, 24) * ((i + 1) as u32);
        result
    })
        .sum()
}

fn solve_part_2(input: &str) -> u32 {
    let blueprints = parse_input(input);

    let first_blueprints = if blueprints.len() >= 3 {
        &blueprints[..3]
    } else {
        &blueprints[..]
    };

    first_blueprints.iter().map(|blueprint| {
        let result = find_max_for_blueprint(blueprint, 32);
        result
    })
        .product()
}

fn find_max_for_blueprint(blueprint: &Blueprint, remaining: u32) -> u32 {
    let initial_state = SearchState::new_initial_state(remaining);
    search(blueprint, initial_state, &mut HashMap::new())
}

fn search(
    blueprint: &Blueprint,
    state: SearchState,
    result_cache: &mut HashMap<SearchState, u32>,
) -> u32 {
    if state.remaining == 0 {
        return 0;
    }

    if let Some(&value) = result_cache.get(&state) {
        return value;
    }

    let SearchState {
        ore,
        ore_robots,
        clay,
        clay_robots,
        obsidian,
        obsidian_robots,
        remaining,
    } = state;

    let next_state = SearchState {
        ore: ore + ore_robots,
        ore_robots,
        clay: clay + clay_robots,
        clay_robots,
        obsidian: obsidian + obsidian_robots,
        obsidian_robots,
        remaining: remaining - 1,
    };

    let mut result = u32::MIN;

    if ore >= blueprint.geode_robot_ore_cost && obsidian >= blueprint.geode_robot_obsidian_cost {
        result = (remaining - 1) + search(
            blueprint,
            SearchState {
                ore: next_state.ore - blueprint.geode_robot_ore_cost,
                obsidian: next_state.obsidian - blueprint.geode_robot_obsidian_cost,
                ..next_state
            },
            result_cache
        )
    } else {
        let max_ore_cost = blueprint.max_ore_cost();
        if ore >= blueprint.ore_robot_ore_cost && ore_robots < max_ore_cost &&
            ore < (1.5 * max_ore_cost as f64).round() as u32
        {
            result = cmp::max(result, search(
                blueprint,
                SearchState {
                    ore: next_state.ore - blueprint.ore_robot_ore_cost,
                    ore_robots: ore_robots + 1,
                    ..next_state
                },
                result_cache
            ))
        }

        if ore >= blueprint.clay_robot_ore_cost && clay_robots < blueprint.obsidian_robot_clay_cost &&
            clay < (1.5 * blueprint.obsidian_robot_clay_cost as f64).round() as u32
        {
            result = cmp::max(result, search(
                blueprint,
                SearchState {
                    ore: next_state.ore - blueprint.clay_robot_ore_cost,
                    clay_robots: clay_robots + 1,
                    ..next_state
                },
                result_cache
            ))
        }

        if ore >= blueprint.obsidian_robot_ore_cost && clay >= blueprint.obsidian_robot_clay_cost &&
            obsidian_robots < blueprint.geode_robot_obsidian_cost && obsidian < (1.5 * blueprint.geode_robot_obsidian_cost as f64).round() as u32
        {
            result = cmp::max(result, search(
                blueprint,
                SearchState {
                    ore: next_state.ore - blueprint.obsidian_robot_ore_cost,
                    clay: next_state.clay - blueprint.obsidian_robot_clay_cost,
                    obsidian_robots: obsidian_robots + 1,
                    ..next_state
                },
                result_cache
            ));
        }

        result = cmp::max(result, search(blueprint, next_state, result_cache));
    }

    result_cache.insert(state, result);
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
    #[ignore] // Takes about 21 seconds with the debug build
    fn test_sample_input_part_2() {
        assert_eq!(3472, solve_part_2(SAMPLE_INPUT));
    }
}