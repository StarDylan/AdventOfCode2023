use std::{collections::HashMap, iter};

use itertools::Itertools;

advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u32> {
    Some(
        input
            .trim()
            .lines()
            .map(|line| {
                let first = line
                    .chars()
                    .find(|c| c.is_ascii_digit())
                    .expect("Numeric Char always exist");

                let last = line
                    .chars()
                    .rev()
                    .find(|c| c.is_ascii_digit())
                    .expect("Numeric Char always exist");

                let full_num = String::from_iter(vec![first, last].iter());

                let parsed_num: u32 = full_num.parse().expect("Number always valid");

                parsed_num
            })
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    let valid_words = HashMap::from([
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
    ]);

    Some(
        input
            .trim()
            .lines()
            .map(|line| {
                // Find all digits and possible words and find the first occurance of one
                let numbers = valid_words
                    .iter()
                    .flat_map(|(word, value)| line.match_indices(word).map(|mtch| (mtch.0, *value)))
                    .chain(line.chars().enumerate().filter_map(|(idx, c)| {
                        if c.is_ascii_digit() {
                            Some((idx, c.to_digit(10).unwrap()))
                        } else {
                            None
                        }
                    }));

                let first = numbers
                    .clone()
                    .min_by_key(|i| i.0)
                    .expect("At least one match")
                    .1;

                let last = numbers
                    .clone()
                    .max_by_key(|i| i.0)
                    .expect("At least one match")
                    .1;

                first * 10 + last
            })
            .sum(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples/part1", DAY));
        assert_eq!(result, Some(142));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples/part2", DAY));
        assert_eq!(result, Some(281));
    }
}
