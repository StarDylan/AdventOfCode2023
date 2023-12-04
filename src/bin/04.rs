use std::collections::HashSet;

use itertools::Itertools;
use once_cell::sync::Lazy;
use regex::Regex;

advent_of_code::solution!(4);

#[derive(Debug)]
struct Card {
    winning_nums: HashSet<u32>,
    numbers_we_have: HashSet<u32>,
}

impl Card {
    fn from_str(s: &str) -> Card {
        static RE: Lazy<Regex> = Lazy::new(|| {
            Regex::new(r"Card\s*(?P<card_num>\d+): (?P<winning_nums>(?:\s*\d+)+) \| (?P<our_nums>(?:\s*\d+)+)")
                    .expect("Valid Regex")
        });

        let caps = RE.captures(s).expect("line Always matches regex");

        Card {
            winning_nums: HashSet::from_iter(
                caps["winning_nums"]
                    .split_ascii_whitespace()
                    .map(|num| num.parse().expect("Valid u32 winning nums")),
            ),

            numbers_we_have: HashSet::from_iter(
                caps["our_nums"]
                    .split_ascii_whitespace()
                    .map(|num| num.parse().expect("Valid u32 our nums")),
            ),
        }
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    Some(
        input
            .lines()
            .map(Card::from_str)
            .map(|card| {
                let winning_nums = card
                    .numbers_we_have
                    .intersection(&card.winning_nums)
                    .count() as u32;

                if winning_nums == 0 {
                    0
                } else {
                    2_u32.pow(winning_nums - 1)
                }
            })
            .sum(),
    )
}

fn process_scratch_card(cards: &Vec<Card>, scratch_card_to_process: usize) -> u32 {
    let card_to_process = cards
        .get(scratch_card_to_process - 1)
        .expect("Valid input to function");

    let wins = card_to_process
        .numbers_we_have
        .intersection(&card_to_process.winning_nums)
        .count();

    if wins == 0 {
        1
    } else {
        // Get next scratch cards
        cards
            .iter()
            .enumerate()
            .skip(scratch_card_to_process)
            .take(wins)
            .map(|(idx, _card)| process_scratch_card(cards, idx + 1))
            .sum::<u32>()
            + 1
    }
}

pub fn part_two(input: &str) -> Option<u32> {
    let cards = input.lines().map(Card::from_str).collect_vec();

    Some(
        (1..=cards.len())
            .map(|card_idx| process_scratch_card(&cards, card_idx))
            .sum(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples/part1", DAY));
        assert_eq!(result, Some(13));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples/part2", DAY));
        assert_eq!(result, Some(30));
    }
}
