use std::{cmp::max, collections::HashMap};

use itertools::Itertools;

advent_of_code::solution!(3);

#[derive(Debug, PartialEq, Eq)]
enum SchematicCell {
    Blank,
    Symbol(char),
    Number(char),

}

#[derive(Debug)]
struct EngineSchematic {
    schematic: Vec<Vec<SchematicCell>>,
}

fn engine_parse(s: &str) -> EngineSchematic {
    let schematic = s.lines().map(|line| {
        line.chars().rev().map(|cell| {
            match cell {
                '.' => SchematicCell::Blank,
                num if cell.is_ascii_digit() => SchematicCell::Number(num),
                symbol if cell.is_ascii_punctuation() => SchematicCell::Symbol(symbol),
                other => panic!("'{other}' is not a valid Schematic Cell Char")

            }
        }).collect_vec()
    }).collect_vec();

    EngineSchematic {
        schematic
    }
}

pub fn part_one(input: &str) -> Option<u32> {

    let nearby_pattern: Vec<(isize, isize)> = vec![
        (-1, 0), (-1, -1), (0, -1), (1, -1), (1, 0), (1, 1), (0, 1), (-1, 1)
    ];
    
    let schematic = engine_parse(input);

    let mut current_number = 0;
    let mut current_digit_idx = 0;

    let mut found_symbol_nearby = false;

    let mut sum = 0;

    for (line_idx, line) in schematic.schematic.iter().enumerate() {
        for (cell_idx, cell) in line.iter().enumerate() {
            match cell {
                SchematicCell::Blank | SchematicCell::Symbol(_) => {
                    if found_symbol_nearby {
                        sum += current_number;
                    }       
                    current_digit_idx = 0;
                    current_number = 0;
                    found_symbol_nearby = false;
                },
                SchematicCell::Number(num) => {
                    // Try to see if there's a symbol nearby
                    found_symbol_nearby = found_symbol_nearby || nearby_pattern.iter().map(|pattern| (pattern.0.saturating_add(line_idx.try_into().unwrap()), pattern.1.saturating_add(cell_idx.try_into().unwrap())))
                        .find(|target: &(isize, isize)| {
                            schematic.schematic.get(TryInto::<usize>::try_into(max(target.0, 0)).unwrap())
                                .and_then(|target_line| target_line.get(TryInto::<usize>::try_into(max(target.1, 0)).unwrap())
                                .map(|cell| matches!(cell, &SchematicCell::Symbol(_))))
                                .unwrap_or(false)
                        }
                    ).is_some();

                    current_number += 10_u32.pow(current_digit_idx) * num.to_digit(10).expect("Always a valid digit");
                    current_digit_idx += 1;
                },
            }
        }
        // Reset the counters on a new line

        if found_symbol_nearby {
            sum += current_number;
        }       
        current_digit_idx = 0;
        current_number = 0;
        found_symbol_nearby = false;
    }

    Some(sum)
}

pub fn part_two(input: &str) -> Option<u32> {
    let nearby_pattern: Vec<(isize, isize)> = vec![
        (-1, 0), (-1, -1), (0, -1), (1, -1), (1, 0), (1, 1), (0, 1), (-1, 1)
    ];
    
    let schematic = engine_parse(input);
    
    let mut current_number: u32 = 0;
    let mut current_digit_idx = 0;

    let mut current_number_indices: Vec<(u32, u32)> = Vec::new();

    let mut number_indices:  Vec<(u32, Vec<(u32, u32)>)> = Vec::new();

    schematic.schematic.iter().enumerate().for_each(|(line_idx, line) | {
        line.iter().enumerate().for_each(|(cell_idx, cell)| {
                match cell {
                    SchematicCell::Blank | SchematicCell::Symbol(_) => {
                        
                        number_indices.push((current_number, current_number_indices.clone()));

                        current_digit_idx = 0;
                        current_number = 0;
                        current_number_indices = Vec::new();

                    },
                    SchematicCell::Number(num) => {
                        // Try to see if there's a symbol nearby
                        current_number += 10_u32.pow(current_digit_idx) * num.to_digit(10).expect("Always a valid digit");
                        current_digit_idx += 1;

                        // Add to current_number_indices
                        current_number_indices.push((line_idx as u32, cell_idx as u32));
                    },
            }
        });
    });

    let mut total_sum = 0;

    // Now we find gear numbers
    schematic.schematic.iter().enumerate().for_each(|(line_idx, line) | {
        line.iter().enumerate().for_each(|(cell_idx, cell)| {
            if let SchematicCell::Symbol('*') = cell {
                // We have a gear, lets try to find part numbers
                let nearby = number_indices.iter()
                    .filter(|num| {
                        // Check if near to this gear
                        num.1.iter().find(|loc| {
                            let dist = (loc.0 as isize - line_idx as isize).pow(2)
                                + (loc.1 as isize - cell_idx as isize).pow(2);

                            if dist <= 2 {
                                // Nearby!
                                return true;
                            }

                            return false;
                        }).is_some()
                    }).collect_vec(); 

                    if nearby.len() == 2 {
                        // We have a valid gear!

                        let first = nearby.get(0).expect("We checked that this is valid").0;
                        let second = nearby.get(1).expect("We checked that this is valid").0;

                        let gear_ratio = first * second;

                        total_sum += gear_ratio;
                    }
            }
        });
    });

    Some(total_sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples/part1", DAY));
        assert_eq!(result, Some(4361));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples/part2", DAY));
        assert_eq!(result, Some(467835));
    }
}
