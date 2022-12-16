use std::cmp;
use std::collections::{HashMap, HashSet, VecDeque};

struct Valve {
    name: String,
    flow_rate: u32,
    tunnels: Vec<String>,
}

struct CaveGraph {
    valves: HashMap<String, Valve>,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct MemoizeKey {
    your_target: String,
    your_remaining_to_target: u32,
    elephant_target: String,
    elephant_remaining_to_target: u32,
    visited: Vec<String>,
    remaining: u32,
    current_total: u32,
    current_running: u32,
    you_stopped: bool,
    elephant_stopped: bool,
}

fn solve(input: &str) -> (u32, u32) {
    let graph = parse_input(input);

    let path_lengths = find_path_lengths(&graph);

    let part_1_solution = find_possible_combinations(&graph, &path_lengths, "AA", HashSet::new(), 30, 0, 0);

    let part_2_solution = cmp::max(part_1_solution, find_with_elephant(
        &graph,
        &path_lengths,
        "AA",
        0,
        "AA",
        0,
        HashSet::new(),
        26,
        0,
        0,
        false,
        false,
        &mut HashMap::new(),
    ));

    (part_1_solution, part_2_solution)
}

fn find_with_elephant(
    graph: &CaveGraph,
    path_lengths: &HashMap<String, HashMap<String, u32>>,
    your_target: &str,
    your_remaining_to_target: u32,
    elephant_target: &str,
    elephant_remaining_to_target: u32,
    visited: HashSet<String>,
    remaining: u32,
    current_total: u32,
    current_running: u32,
    you_stopped: bool,
    elephant_stopped: bool,
    cache: &mut HashMap<MemoizeKey, u32>,
) -> u32 {
    // let memoize_key = MemoizeKey {
    //     your_target: String::from(your_target),
    //     your_remaining_to_target,
    //     elephant_target: String::from(elephant_target),
    //     elephant_remaining_to_target,
    //     visited: visited.iter().cloned().collect(),
    //     remaining,
    //     current_total,
    //     current_running,
    //     you_stopped,
    //     elephant_stopped,
    // };
    // if let Some(&result) = cache.get(&memoize_key) {
    //     return result;
    // }


    let mut result = current_total + remaining * current_running;

    // println!("STATUS:");
    // println!("  your_target: {your_target}, remaining={remaining}");
    // println!("  your_remaining: {your_remaining_to_target}");
    // println!("  elephant_target: {elephant_target}, remaining={remaining}");
    // println!("  elephant_remaining: {elephant_remaining_to_target}");
    // println!("  visited: {visited:?}");
    // println!("  remaining: {remaining}");
    // println!("  current_total: {current_total}");
    // println!("  current_running: {current_running}");
    // println!("  current max: {result}");
    // println!();

    if your_remaining_to_target == 0 && !you_stopped {
        let add_to_running = graph.valves.get(your_target).unwrap().flow_rate;

        let mut moved = false;

        for (other_name, &distance) in path_lengths.get(your_target).unwrap() {
            if !visited.contains(other_name) && distance + 2 <= remaining {
                let mut new_visited = visited.clone();
                new_visited.insert(other_name.clone());

                let sub_result = find_with_elephant(
                    graph,
                    path_lengths,
                    other_name,
                    distance + 1,
                    elephant_target,
                    elephant_remaining_to_target,
                    new_visited,
                    remaining,
                    current_total,
                    current_running + add_to_running,
                    false,
                    elephant_stopped,
                    cache,
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
                visited.clone(),
                remaining,
                current_total,
                current_running + add_to_running,
                true,
                elephant_stopped,
                cache,
            );
            result = cmp::max(result, sub_result);
        }
    } else if elephant_remaining_to_target == 0 && !elephant_stopped {
        let add_to_running = graph.valves.get(elephant_target).unwrap().flow_rate;

        let mut moved = false;

        for (other_name, &distance) in path_lengths.get(elephant_target).unwrap() {
            if !visited.contains(other_name) && distance + 2 <= remaining {
                let mut new_visited = visited.clone();
                new_visited.insert(other_name.clone());

                let sub_result = find_with_elephant(
                    graph,
                    path_lengths,
                    your_target,
                    your_remaining_to_target,
                    other_name,
                    distance + 1,
                    new_visited,
                    remaining,
                    current_total,
                    current_running + add_to_running,
                    you_stopped,
                    false,
                    cache,
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
                visited.clone(),
                remaining,
                current_total,
                current_running + add_to_running,
                you_stopped,
                true,
                cache,
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
            visited.clone(),
            remaining - your_remaining_to_target,
            current_total + your_remaining_to_target * current_running,
            current_running,
            false,
            elephant_stopped,
            cache,
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
            visited.clone(),
            remaining - elephant_remaining_to_target,
            current_total + elephant_remaining_to_target * current_running,
            current_running,
            you_stopped,
            false,
            cache,
        );
        result = cmp::max(result, sub_result);
    }

    // cache.insert(memoize_key, result);

    result
}

fn find_possible_combinations(graph: &CaveGraph, path_lengths: &HashMap<String, HashMap<String, u32>>, start: &str, visited: HashSet<String>, remaining: u32, current_total: u32, current_running: u32) -> u32 {
    let mut result = current_total + remaining * current_running;

    for (other_name, distance) in path_lengths.get(start).unwrap() {
        if !visited.contains(other_name) && distance + 2 <= remaining {
            let mut new_visited = visited.clone();
            new_visited.insert(String::from(start));

            let sub_result = find_possible_combinations(
                graph,
                path_lengths,
                other_name,
                new_visited,
                remaining - distance - 1,
                current_total + (distance + 1) * current_running,
                current_running + graph.valves.get(other_name).unwrap().flow_rate,
            );
            result = cmp::max(result, sub_result);
        }
    }

    result
}

fn find_path_lengths(graph: &CaveGraph) -> HashMap<String, HashMap<String, u32>> {
    let mut result: HashMap<String, HashMap<String, u32>> = HashMap::new();

    for valve in graph.valves.values().filter(|valve| valve.name == "AA" || valve.flow_rate > 0) {
        let mut valve_map: HashMap<String, u32> = HashMap::new();
        for other_valve in graph.valves.values().filter(|other_valve| valve.name != other_valve.name && other_valve.flow_rate > 0) {
            let distance = find_shortest_path(graph, &valve.name, &other_valve.name);
            valve_map.insert(other_valve.name.clone(), distance);
        }
        result.insert(valve.name.clone(), valve_map);
    }

    result
}

fn find_shortest_path(graph: &CaveGraph, a: &str, b: &str) -> u32 {
    let mut queue: VecDeque<(&str, u32)> = VecDeque::new();
    queue.push_back((a, 0));

    let mut visited: HashSet<String> = HashSet::new();
    visited.insert(String::from(a));

    while !queue.is_empty() {
        let (name, distance) = queue.pop_front().unwrap();

        for tunnel in &graph.valves.get(name).unwrap().tunnels {
            if tunnel == b {
                return distance + 1;
            }

            if !visited.contains(tunnel) {
                visited.insert(String::from(tunnel));
                queue.push_back((tunnel, distance + 1));
            }
        }
    }

    panic!("no path found from {a} to {b}");
}

fn parse_input(input: &str) -> CaveGraph {
    let valves: HashMap<_, _> = input.lines().map(|line| {
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
            .map(String::from)
            .collect();
        (name.clone(), Valve { name, flow_rate, tunnels })
    })
        .collect();

    CaveGraph { valves }
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