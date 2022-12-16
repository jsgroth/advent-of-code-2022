//! Day 16: Proboscidea Volcanium
//! https://adventofcode.com/2022/day/16

use std::cmp;
use std::collections::{HashMap, HashSet, VecDeque};

struct Valve {
    name: String,
    index: usize,
    flow_rate: u32,
    tunnels: Vec<usize>,
}

impl Valve {
    fn new(name: String, index: usize) -> Self {
        Self {
            name,
            index,
            flow_rate: 0,
            tunnels: Vec::new(),
        }
    }
}

struct CaveGraph {
    valves: Vec<Valve>,
    name_to_index: HashMap<String, usize>,
    valves_with_flow: Vec<usize>,
}

impl CaveGraph {
    fn new() -> Self {
        Self {
            valves: Vec::new(),
            name_to_index: HashMap::new(),
            valves_with_flow: Vec::new(),
        }
    }

    fn get_or_add_valve_index(&mut self, name: &str) -> usize {
        if let Some(&index) = self.name_to_index.get(name) {
            index
        } else {
            let index = self.valves.len();
            self.valves.push(Valve::new(String::from(name), index));
            self.name_to_index.insert(String::from(name), index);
            index
        }
    }

    fn add_valve(&mut self, name: &str, flow_rate: u32, tunnels: &[&str]) {
        let index = self.get_or_add_valve_index(name);
        self.valves[index].flow_rate = flow_rate;

        for tunnel in tunnels {
            let tunnel_index = self.get_or_add_valve_index(tunnel);
            self.valves[index].tunnels.push(tunnel_index);
        }

        if flow_rate > 0 {
            self.valves_with_flow.push(index);
        }
    }
}

fn solve(input: &str) -> (u32, u32) {
    let graph = parse_input(input);

    let path_lengths = find_path_lengths(&graph);

    let start_index = *graph.name_to_index.get("AA").unwrap();
    let part_1_solution = find_possible_combinations(&graph, &path_lengths, start_index, HashSet::new(), 30, 0, 0);

    let part_2_solution = cmp::max(part_1_solution, find_with_elephant(
        &graph,
        &path_lengths,
        start_index,
        0,
        start_index,
        0,
        HashSet::new(),
        26,
        0,
        0,
        false,
        false,
        &mut 0,
    ));

    (part_1_solution, part_2_solution)
}

fn find_with_elephant(
    graph: &CaveGraph,
    path_lengths: &Vec<Vec<u32>>,
    your_target: usize,
    your_remaining_to_target: u32,
    elephant_target: usize,
    elephant_remaining_to_target: u32,
    visited: HashSet<usize>,
    remaining: u32,
    current_total: u32,
    current_running: u32,
    you_stopped: bool,
    elephant_stopped: bool,
    max_so_far: &mut u32,
) -> u32 {
    if *max_so_far >= compute_max_possible(
        graph,
        path_lengths,
        your_target,
        your_remaining_to_target,
        elephant_target,
        elephant_remaining_to_target,
        &visited,
        remaining,
        current_total,
        current_running,
        you_stopped,
        elephant_stopped
    ) {
        return u32::MIN;
    }

    let mut result = current_total + remaining * current_running;

    if your_remaining_to_target == 0 && !you_stopped {
        let add_to_running = graph.valves[your_target].flow_rate;

        let mut moved = false;

        for &other_index in &graph.valves_with_flow {
            let distance = path_lengths[your_target][other_index];
            if !visited.contains(&other_index) && distance + 2 <= remaining {
                let mut new_visited = visited.clone();
                new_visited.insert(other_index);

                let sub_result = find_with_elephant(
                    graph,
                    path_lengths,
                    other_index,
                    distance + 1,
                    elephant_target,
                    elephant_remaining_to_target,
                    new_visited,
                    remaining,
                    current_total,
                    current_running + add_to_running,
                    false,
                    elephant_stopped,
                    max_so_far,
                );
                result = cmp::max(result, sub_result);

                moved = true;
            }
        }

        if !moved {
            let sub_result = find_with_elephant(
                graph,
                path_lengths,
                your_target,
                0,
                elephant_target,
                elephant_remaining_to_target,
                visited,
                remaining,
                current_total,
                current_running + add_to_running,
                true,
                elephant_stopped,
                max_so_far,
            );
            result = cmp::max(result, sub_result);
        }
    } else if elephant_remaining_to_target == 0 && !elephant_stopped {
        let add_to_running = graph.valves[elephant_target].flow_rate;

        let mut moved = false;

        for &other_index in &graph.valves_with_flow {
            let distance = path_lengths[elephant_target][other_index];
            if !visited.contains(&other_index) && distance + 2 <= remaining {
                let mut new_visited = visited.clone();
                new_visited.insert(other_index);

                let sub_result = find_with_elephant(
                    graph,
                    path_lengths,
                    your_target,
                    your_remaining_to_target,
                    other_index,
                    distance + 1,
                    new_visited,
                    remaining,
                    current_total,
                    current_running + add_to_running,
                    you_stopped,
                    false,
                    max_so_far,
                );
                result = cmp::max(result, sub_result);

                moved = true;
            }
        }

        if !moved {
            let sub_result = find_with_elephant(
                graph,
                path_lengths,
                your_target,
                your_remaining_to_target,
                elephant_target,
                0,
                visited,
                remaining,
                current_total,
                current_running + add_to_running,
                you_stopped,
                true,
                max_so_far,
            );
            result = cmp::max(result, sub_result);
        }
    } else if your_remaining_to_target > 0 && (elephant_stopped || your_remaining_to_target <= elephant_remaining_to_target) {
        let new_elephant_remaining = if !elephant_stopped { elephant_remaining_to_target - your_remaining_to_target } else { 0 };
        let sub_result = find_with_elephant(
            graph,
            path_lengths,
            your_target,
            0,
            elephant_target,
            new_elephant_remaining,
            visited,
            remaining - your_remaining_to_target,
            current_total + your_remaining_to_target * current_running,
            current_running,
            false,
            elephant_stopped,
            max_so_far,
        );
        result = cmp::max(result, sub_result);
    } else if elephant_remaining_to_target > 0 && (you_stopped || elephant_remaining_to_target <= your_remaining_to_target) {
        let new_your_remaining = if !you_stopped { your_remaining_to_target - elephant_remaining_to_target } else { 0 };
        let sub_result = find_with_elephant(
            graph,
            path_lengths,
            your_target,
            new_your_remaining,
            elephant_target,
            0,
            visited,
            remaining - elephant_remaining_to_target,
            current_total + elephant_remaining_to_target * current_running,
            current_running,
            you_stopped,
            false,
            max_so_far,
        );
        result = cmp::max(result, sub_result);
    }

    *max_so_far = cmp::max(*max_so_far, result);

    result
}

fn compute_max_possible(
    graph: &CaveGraph,
    path_lengths: &Vec<Vec<u32>>,
    your_target: usize,
    your_remaining_to_target: u32,
    elephant_target: usize,
    elephant_remaining_to_target: u32,
    visited: &HashSet<usize>,
    remaining: u32,
    current_total: u32,
    current_running: u32,
    you_stopped: bool,
    elephant_stopped: bool,
) -> u32 {
    let mut total = current_total + remaining * current_running;

    if !you_stopped {
        total += (remaining - your_remaining_to_target) * graph.valves[your_target].flow_rate;
    }
    if !elephant_stopped {
        total += (remaining - elephant_remaining_to_target) * graph.valves[elephant_target].flow_rate;
    }

    let unvisited_indices: Vec<_> = graph.valves.iter().filter_map(|valve| {
        if valve.flow_rate > 0 && !visited.contains(&valve.index) {
            Some(valve.index)
        } else {
            None
        }
    }).collect();

    for &index in &unvisited_indices {
        if !visited.contains(&index) {
            let earliest_possible = cmp::min(
                if !you_stopped { your_remaining_to_target + path_lengths[your_target][index] + 1 } else { u32::MAX },
                if !elephant_stopped { elephant_remaining_to_target + path_lengths[elephant_target][index] + 1 } else { u32::MAX },
            );
            total += remaining.saturating_sub(earliest_possible) * graph.valves[index].flow_rate;
        }
    }

    total
}

fn find_possible_combinations(graph: &CaveGraph, path_lengths: &Vec<Vec<u32>>, start: usize, visited: HashSet<usize>, remaining: u32, current_total: u32, current_running: u32) -> u32 {
    let mut result = current_total + remaining * current_running;

    for &other_index in &graph.valves_with_flow {
        let distance = path_lengths[start][other_index];
        if !visited.contains(&other_index) && distance + 2 <= remaining {
            let mut new_visited = visited.clone();
            new_visited.insert(other_index);

            let sub_result = find_possible_combinations(
                graph,
                path_lengths,
                other_index,
                new_visited,
                remaining - distance - 1,
                current_total + (distance + 1) * current_running,
                current_running + graph.valves[other_index].flow_rate,
            );
            result = cmp::max(result, sub_result);
        }
    }

    result
}

fn find_path_lengths(graph: &CaveGraph) -> Vec<Vec<u32>> {
    let mut result = vec![vec![0; graph.valves.len()]; graph.valves.len()];

    for valve in graph.valves.iter().filter(|valve| valve.name == "AA" || valve.flow_rate > 0) {
        for other_valve in graph.valves.iter().filter(|other_valve| valve.index != other_valve.index && other_valve.flow_rate > 0) {
            let distance = find_shortest_path(graph, valve.index, other_valve.index);
            result[valve.index][other_valve.index] = distance;
        }
    }

    result
}

fn find_shortest_path(graph: &CaveGraph, a: usize, b: usize) -> u32 {
    let mut queue: VecDeque<(usize, u32)> = VecDeque::new();
    queue.push_back((a, 0));

    let mut visited: HashSet<usize> = HashSet::new();
    visited.insert(a);

    while !queue.is_empty() {
        let (index, distance) = queue.pop_front().unwrap();

        for &tunnel in &graph.valves[index].tunnels {
            if tunnel == b {
                return distance + 1;
            }

            if !visited.contains(&tunnel) {
                visited.insert(tunnel);
                queue.push_back((tunnel, distance + 1));
            }
        }
    }

    panic!("no path found from {a} to {b}");
}

fn parse_input(input: &str) -> CaveGraph {
    let mut graph = CaveGraph::new();

    for line in input.lines() {
        let mut split = line.split(' ').skip(1);
        let name = split.next().expect("valve name");
        let name = String::from(name);

        split.next();
        split.next();

        let flow_rate = split.next().expect("flow rate");
        let flow_rate = flow_rate["rate=".len()..flow_rate.len() - 1].parse().expect("flow rate should be an integer");

        let tunnels: Vec<_> = split.skip(4).map(|tunnel| {
            if tunnel.chars().last() == Some(',') {
                &tunnel[..tunnel.len() - 1]
            } else {
                tunnel
            }
        })
            .collect();

        graph.add_valve(&name, flow_rate, &tunnels);
    }

    graph
}

fn main() {
    let input = advent_of_code_2022::read_input().expect("unable to read input file");

    let (solution1, solution2) = solve(&input);
    println!("{solution1}");
    println!("{solution2}");
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_INPUT: &str = include_str!("sample_input/sample16.txt");

    #[test]
    fn test_sample_input() {
        assert_eq!((1651, 1707), solve(SAMPLE_INPUT));
    }
}