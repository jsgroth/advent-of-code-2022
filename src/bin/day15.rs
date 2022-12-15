//! Day 15: Beacon Exclusion Zone
//! https://adventofcode.com/2022/day/15

use std::collections::HashSet;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
}

#[derive(Debug)]
struct Sensor {
    coordinates: Point,
    nearest_beacon: Point,
    nearest_beacon_distance: i32,
}

impl Sensor {
    fn new(coordinates: Point, nearest_beacon: Point) -> Self {
        let nearest_beacon_distance = (coordinates.x - nearest_beacon.x).abs() + (coordinates.y - nearest_beacon.y).abs();
        Self { coordinates, nearest_beacon, nearest_beacon_distance }
    }

    fn distance_to(&self, p: &Point) -> i32 {
        (self.coordinates.x - p.x).abs() + (self.coordinates.y - p.y).abs()
    }
}

const PART_1_ROW_TO_CHECK: i32 = 2000000;

const PART_2_MAX_COORDINATE: i32 = 4000000;

fn solve(input: &str, y_to_check: i32) -> usize {
    let sensors = parse_input(input);

    let min_x = sensors.iter().map(|sensor| {
        sensor.coordinates.x - sensor.nearest_beacon_distance
    })
        .min()
        .unwrap();

    let max_x = sensors.iter().map(|sensor| {
        sensor.coordinates.x + sensor.nearest_beacon_distance
    })
        .max()
        .unwrap();

    let beacon_positions: HashSet<_> = sensors.iter().map(|sensor| sensor.nearest_beacon.clone()).collect();

    (min_x..=max_x).filter(|&x| {
        let p = Point::new(x, y_to_check);
        !beacon_positions.contains(&p) && sensors.iter().any(|sensor| {
            sensor.distance_to(&p) <= sensor.nearest_beacon_distance
        })
    }).count()
}

fn solve_part_2(input: &str, max_coordinate: i32) -> i64 {
    let sensors = parse_input(input);

    for sensor in &sensors {
        let points = generate_candidate_points(sensor, max_coordinate);
        let outside_all_sensors: Vec<_> = points.into_iter().filter(|p| {
            sensors.iter().all(|sensor| sensor.distance_to(p) > sensor.nearest_beacon_distance)
        }).collect();

        if outside_all_sensors.len() == 1 {
            let result = &outside_all_sensors[0];
            return (result.x as i64) * 4000000 + (result.y as i64)
        }
    }

    panic!("no solution found");
}

// Return all points that are (nearest_beacon_distance + 1) away from the sensor and within bounds
fn generate_candidate_points(sensor: &Sensor, max_coordinate: i32) -> Vec<Point> {
    let mut result: Vec<Point> = Vec::new();

    let mut p = Point::new(sensor.coordinates.x - sensor.nearest_beacon_distance - 1, sensor.coordinates.y);
    while p.x != sensor.coordinates.x {
        if in_bounds(&p, max_coordinate) {
            result.push(p);
        }

        p.x += 1;
        p.y += 1;
    }

    while p.y != sensor.coordinates.y {
        if in_bounds(&p, max_coordinate) {
            result.push(p);
        }

        p.x += 1;
        p.y -= 1;
    }

    while p.x != sensor.coordinates.x {
        if in_bounds(&p, max_coordinate) {
            result.push(p);
        }

        p.x -= 1;
        p.y -= 1;
    }

    while p.y != sensor.coordinates.y {
        if in_bounds(&p, max_coordinate) {
            result.push(p);
        }

        p.x -= 1;
        p.y += 1;
    }

    result
}

fn in_bounds(p: &Point, max_coordinate: i32) -> bool {
    p.x >= 0 && p.y >= 0 && p.x <= max_coordinate && p.y <= max_coordinate
}

fn parse_input(input: &str) -> Vec<Sensor> {
    input.lines().map(|line| {
        let colon_index = line.find(':').expect("line should have a colon");

        let coordinates = parse_coordinates(&line["Sensor at ".len()..colon_index]);

        let nearest_beacon_index = colon_index + ": closest beacon is at ".len();
        let nearest_beacon = parse_coordinates(&line[nearest_beacon_index..]);

        Sensor::new(coordinates, nearest_beacon)
    })
        .collect()
}

fn parse_coordinates(coordinates: &str) -> Point {
    let (x, y) = coordinates.split_once(", ").expect("coordinates should have a comma");
    let x: i32 = x["x=".len()..].parse().expect("x coordinate should be an integer");
    let y: i32 = y["y=".len()..].parse().expect("y coordinate should be an integer");

    Point::new(x, y)
}

fn main() {
    let input = advent_of_code_2022::read_input().expect("unable to read input file");

    let solution1 = solve(&input, PART_1_ROW_TO_CHECK);
    println!("{solution1}");

    let solution2 = solve_part_2(&input, PART_2_MAX_COORDINATE);
    println!("{solution2}");
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_INPUT: &str = include_str!("sample_input/sample15.txt");

    #[test]
    fn test_sample_input_part_1() {
        assert_eq!(26, solve(SAMPLE_INPUT, 10));
    }

    #[test]
    fn test_sample_input_part_2() {
        assert_eq!(56000011, solve_part_2(SAMPLE_INPUT, 20));
    }
}