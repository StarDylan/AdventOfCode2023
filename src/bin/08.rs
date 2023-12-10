use std::collections::HashMap;

use itertools::Itertools;
use once_cell::sync::Lazy;
use regex::Regex;

advent_of_code::solution!(8);

#[derive(Debug)]
struct NodeDirection {
    left: String,
    right: String,
}

#[derive(Debug)]
struct Map {
    directions: Vec<Direction>,
    map: HashMap<String, NodeDirection>,
}

#[derive(Debug)]
enum Direction {
    Left,
    Right,
}

const START: &str = "AAA";
const END: &str = "ZZZ";

fn file_to_node_map(s: &str) -> Map {
    let mut lines = s.lines();

    let directions = lines
        .next()
        .expect("Always a first line")
        .chars()
        .map(|char| match char {
            'L' => Direction::Left,
            'R' => Direction::Right,
            _ => panic!("Invalid Direction Char {}", char),
        })
        .collect_vec();

    let lines = lines.skip(1); // Empty Line

    static LINE_MAP_RE: Lazy<Regex> = Lazy::new(|| {
        Regex::new(r"(?P<node>\w{3}) = \((?P<left>\w{3}), (?P<right>\w{3})\)").expect("Valid Regex")
    });
    let map = lines
        .map(|line| {
            let matches = LINE_MAP_RE
                .captures(line)
                .expect("Every line contains a valid map line");

            (
                matches["node"].to_string(),
                NodeDirection {
                    left: matches["left"].to_string(),
                    right: matches["right"].to_string(),
                },
            )
        })
        .collect();

    Map { directions, map }
}

pub fn part_one(input: &str) -> Option<u32> {
    let map = file_to_node_map(input);

    let mut current_node = START;
    let mut directions = map.directions.iter().cycle();

    let mut count = 0;

    while current_node != END {
        let next_dir = directions.next().expect("Cycled Iter never ends");

        let potential_directions = map.map.get(current_node).expect("Node always exists");

        match next_dir {
            Direction::Left => current_node = &potential_directions.left,
            Direction::Right => current_node = &potential_directions.right,
        }

        count += 1;
    }

    Some(count)
}

pub fn part_two(input: &str) -> Option<usize> {
    let map = file_to_node_map(input);

    let current_nodes = map
        .map
        .keys()
        .filter(|node| node.ends_with('A'))
        .cloned()
        .collect_vec();

    // Key insight: All paths are cycles.

    let path_cycle_lengths = current_nodes
        .iter()
        .map(|node| {
            let mut current_node = node;
            let mut directions = map.directions.iter().cycle();
            let mut count: usize = 0;

            while !current_node.ends_with('Z') {
                let next_dir = directions.next().expect("Cycled Iter never ends");

                let potential_directions = map.map.get(current_node).expect("Node always exists");

                match next_dir {
                    Direction::Left => current_node = &potential_directions.left,
                    Direction::Right => current_node = &potential_directions.right,
                }

                count += 1;
            }

            count
        })
        .collect_vec();

    Some(
        path_cycle_lengths
            .iter()
            .fold(1usize, |acc, len| num::integer::lcm(acc, *len)),
    )
}

//11678319315857

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples/part1", DAY));
        assert_eq!(result, Some(6));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples/part2", DAY));
        assert_eq!(result, Some(6));
    }
}
