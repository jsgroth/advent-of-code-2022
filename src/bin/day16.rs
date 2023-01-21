//! Day 16: Proboscidea Volcanium
//!
//! <https://adventofcode.com/2022/day/16>

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

#[derive(Clone)]
struct ElephantFindParameters<'a> {
    graph: &'a CaveGraph,
    path_lengths: &'a Vec<Vec<u32>>,
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
}

impl<'a> ElephantFindParameters<'a> {
    fn new(
        graph: &'a CaveGraph,
        path_lengths: &'a Vec<Vec<u32>>,
        you_start: usize,
        elephant_start: usize,
        remaining: u32,
    ) -> Self {
        Self {
            graph,
            path_lengths,
            your_target: you_start,
            your_remaining_to_target: 0,
            elephant_target: elephant_start,
            elephant_remaining_to_target: 0,
            visited: HashSet::new(),
            remaining,
            current_total: 0,
            current_running: 0,
            you_stopped: false,
            elephant_stopped: false,
        }
    }
}

const START_VALVE_NAME: &str = "AA";

const TURN_LIMIT_WITHOUT_ELEPHANT: u32 = 30;
const TURN_LIMIT_WITH_ELEPHANT: u32 = 26;

fn solve(input: &str) -> (u32, u32) {
    let graph = parse_input(input);

    let path_lengths = find_path_lengths(&graph, START_VALVE_NAME);

    let start_index = *graph.name_to_index.get(START_VALVE_NAME).unwrap();
    let part_1_solution = find_best_path(
        &graph,
        &path_lengths,
        start_index,
        HashSet::new(),
        TURN_LIMIT_WITHOUT_ELEPHANT,
        0,
        0,
    );

    let elephant_find_parameters = ElephantFindParameters::new(
        &graph,
        &path_lengths,
        start_index,
        start_index,
        TURN_LIMIT_WITH_ELEPHANT,
    );
    let part_2_solution = cmp::max(
        part_1_solution,
        find_with_elephant(elephant_find_parameters, &mut 0),
    );

    (part_1_solution, part_2_solution)
}

// Find the distance between each pair of nodes with flow, as well as the distance from the
// starting node to every node with flow
fn find_path_lengths(graph: &CaveGraph, start_node: &str) -> Vec<Vec<u32>> {
    let mut result = vec![vec![0; graph.valves.len()]; graph.valves.len()];

    for valve in graph
        .valves
        .iter()
        .filter(|valve| valve.name == start_node || valve.flow_rate > 0)
    {
        for other_valve in graph
            .valves
            .iter()
            .filter(|other_valve| valve.index != other_valve.index && other_valve.flow_rate > 0)
        {
            let distance = find_shortest_path(graph, valve.index, other_valve.index);
            result[valve.index][other_valve.index] = distance;
        }
    }

    result
}

// Simple BFS to find the length of the shortest path between two nodes
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

fn find_best_path(
    graph: &CaveGraph,
    path_lengths: &Vec<Vec<u32>>,
    start: usize,
    visited: HashSet<usize>,
    remaining: u32,
    current_total: u32,
    current_running: u32,
) -> u32 {
    let mut result = current_total + remaining * current_running;

    for &other_index in &graph.valves_with_flow {
        let distance = path_lengths[start][other_index];
        if !visited.contains(&other_index) && distance + 2 <= remaining {
            let mut new_visited = visited.clone();
            new_visited.insert(other_index);

            let sub_result = find_best_path(
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

fn find_with_elephant(parameters: ElephantFindParameters, max_so_far: &mut u32) -> u32 {
    if *max_so_far >= compute_max_possible(&parameters) {
        // Break early, the upper bound for this path is lower than the best we've seen so far
        return u32::MIN;
    }

    let ElephantFindParameters {
        graph,
        path_lengths,
        your_target,
        your_remaining_to_target,
        elephant_target,
        elephant_remaining_to_target,
        remaining,
        current_total,
        current_running,
        you_stopped,
        elephant_stopped,
        ..
    } = parameters;
    let visited = &parameters.visited;

    let mut result = current_total + remaining * current_running;

    if your_remaining_to_target == 0 && !you_stopped {
        // Recursively search all possible new targets for you, as well as the possibility where you stop here

        let add_to_running = graph.valves[your_target].flow_rate;

        for &other_index in &graph.valves_with_flow {
            let distance = path_lengths[your_target][other_index];
            if !visited.contains(&other_index) && distance + 2 <= remaining {
                let mut new_parameters = ElephantFindParameters {
                    your_target: other_index,
                    your_remaining_to_target: distance + 1,
                    current_running: current_running + add_to_running,
                    ..parameters.clone()
                };
                new_parameters.visited.insert(other_index);

                result = cmp::max(result, find_with_elephant(new_parameters, max_so_far));
            }
        }

        let new_parameters = ElephantFindParameters {
            current_running: current_running + add_to_running,
            you_stopped: true,
            ..parameters
        };

        result = cmp::max(result, find_with_elephant(new_parameters, max_so_far));
    } else if elephant_remaining_to_target == 0 && !elephant_stopped {
        // Recursively search all possible new targets for the elephant, as well as the possibility where it stops here

        let add_to_running = graph.valves[elephant_target].flow_rate;

        for &other_index in &graph.valves_with_flow {
            let distance = path_lengths[elephant_target][other_index];
            if !visited.contains(&other_index) && distance + 2 <= remaining {
                let mut new_parameters = ElephantFindParameters {
                    elephant_target: other_index,
                    elephant_remaining_to_target: distance + 1,
                    current_running: current_running + add_to_running,
                    ..parameters.clone()
                };
                new_parameters.visited.insert(other_index);

                result = cmp::max(result, find_with_elephant(new_parameters, max_so_far));
            }
        }

        let new_parameters = ElephantFindParameters {
            current_running: current_running + add_to_running,
            elephant_stopped: true,
            ..parameters
        };

        result = cmp::max(result, find_with_elephant(new_parameters, max_so_far));
    } else if your_remaining_to_target > 0
        && (elephant_stopped || your_remaining_to_target <= elephant_remaining_to_target)
    {
        // Advance time so that you reach your current target

        let new_parameters = ElephantFindParameters {
            your_remaining_to_target: 0,
            elephant_remaining_to_target: if !elephant_stopped {
                elephant_remaining_to_target - your_remaining_to_target
            } else {
                0
            },
            remaining: remaining - your_remaining_to_target,
            current_total: current_total + your_remaining_to_target * current_running,
            ..parameters
        };

        result = cmp::max(result, find_with_elephant(new_parameters, max_so_far));
    } else if elephant_remaining_to_target > 0
        && (you_stopped || elephant_remaining_to_target <= your_remaining_to_target)
    {
        // Advance time so that the elephant reaches its current target

        let new_parameters = ElephantFindParameters {
            your_remaining_to_target: if !you_stopped {
                your_remaining_to_target - elephant_remaining_to_target
            } else {
                0
            },
            elephant_remaining_to_target: 0,
            remaining: remaining - elephant_remaining_to_target,
            current_total: current_total + elephant_remaining_to_target * current_running,
            ..parameters
        };

        result = cmp::max(result, find_with_elephant(new_parameters, max_so_far));
    }

    *max_so_far = cmp::max(*max_so_far, result);

    result
}

// Determine an upper bound for the max possible result from this path
fn compute_max_possible(parameters: &ElephantFindParameters) -> u32 {
    let &ElephantFindParameters {
        graph,
        path_lengths,
        your_target,
        your_remaining_to_target,
        elephant_target,
        elephant_remaining_to_target,
        remaining,
        current_total,
        current_running,
        you_stopped,
        elephant_stopped,
        ..
    } = parameters;
    let visited = &parameters.visited;

    let mut total = current_total + remaining * current_running;

    if !you_stopped {
        total += (remaining - your_remaining_to_target) * graph.valves[your_target].flow_rate;
    }
    if !elephant_stopped {
        total +=
            (remaining - elephant_remaining_to_target) * graph.valves[elephant_target].flow_rate;
    }

    let unvisited_indices: Vec<_> = graph
        .valves_with_flow
        .iter()
        .copied()
        .filter(|index| !visited.contains(index))
        .collect();

    for &index in &unvisited_indices {
        let earliest_possible = cmp::min(
            if !you_stopped {
                your_remaining_to_target + path_lengths[your_target][index] + 1
            } else {
                u32::MAX
            },
            if !elephant_stopped {
                elephant_remaining_to_target + path_lengths[elephant_target][index] + 1
            } else {
                u32::MAX
            },
        );
        total += remaining.saturating_sub(earliest_possible) * graph.valves[index].flow_rate;
    }

    total
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
        let flow_rate = flow_rate["rate=".len()..flow_rate.len() - 1]
            .parse()
            .expect("flow rate should be an integer");

        let tunnels: Vec<_> = split
            .skip(4)
            .map(|tunnel| tunnel.strip_suffix(',').unwrap_or(tunnel))
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
    fn test_sample_input_part_1() {
        let (solution1, _) = solve(SAMPLE_INPUT);
        assert_eq!(1651, solution1);
    }

    #[test]
    fn test_sample_input_part_2() {
        let (_, solution2) = solve(SAMPLE_INPUT);
        assert_eq!(1707, solution2);
    }
}
