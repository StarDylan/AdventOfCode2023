use std::collections::HashSet;

use itertools::Itertools;

advent_of_code::solution!(7);

const CARD_ORDERING: &str = "AKQJT98765432";
const CARD_ORDERING2: &str = "AKQT98765432J";

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Part {
    Part1,
    Part2,
}

#[derive(Debug)]
struct Hand {
    cards: Vec<char>,
    pub bid: u32,
    ordering: Part,
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let classification_self = if self.ordering == Part::Part1 {
            HandClassification::classify_hand(&self.cards)
        } else {
            HandClassification::classify_hand_with_jokers(&self.cards)
        };

        let other_classification = if self.ordering == Part::Part1 {
            HandClassification::classify_hand(&other.cards)
        } else {
            HandClassification::classify_hand_with_jokers(&other.cards)
        };

        if classification_self == other_classification {
            // Do linear card search
            let numbered_cards = self
                .cards
                .iter()
                .zip(other.cards.iter())
                .map(|(card_self, card_other)| {
                    (
                        (if self.ordering == Part::Part1 {
                            CARD_ORDERING
                        } else {
                            CARD_ORDERING2
                        })
                        .chars()
                        .position(|order_card| order_card == *card_self)
                        .expect("Always in CARD_ORDERING"),
                        (if self.ordering == Part::Part1 {
                            CARD_ORDERING
                        } else {
                            CARD_ORDERING2
                        })
                        .chars()
                        .position(|order_card| order_card == *card_other)
                        .expect("Always in CARD_ORDERING"),
                    )
                })
                .collect_vec();

            for (self_card, other_card) in numbered_cards {
                match self_card.cmp(&other_card) {
                    std::cmp::Ordering::Less | std::cmp::Ordering::Greater => {
                        return self_card.cmp(&other_card)
                    }
                    std::cmp::Ordering::Equal => {}
                }
            }

            // Must be equal
            return std::cmp::Ordering::Equal;
        }
        classification_self.cmp(&other_classification)
    }
}

impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other).is_eq()
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl Eq for Hand {}

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug)]
enum HandClassification {
    FiveOfAKind,
    FourOfAKind,
    FullHouse,
    ThreeOfAKind,
    TwoPair,
    OnePair,
    HighCard,
}

impl HandClassification {
    pub fn classify_hand(cards: &[char]) -> Self {
        let unique_cards: HashSet<char> = HashSet::from_iter(cards.iter().cloned());

        if unique_cards.len() == 1 {
            return Self::FiveOfAKind;
        }

        if unique_cards.len() == 2 {
            let mut number_of_each_card = unique_cards
                .iter()
                .map(|card_value| cards.iter().filter(|card| **card == *card_value).count())
                .collect_vec();

            number_of_each_card.sort();

            if number_of_each_card == vec![1, 4] {
                return Self::FourOfAKind;
            }

            if number_of_each_card == vec![2, 3] {
                return Self::FullHouse;
            }
        }

        if unique_cards.len() == 3 {
            let mut number_of_each_card = unique_cards
                .iter()
                .map(|card_value| cards.iter().filter(|card| **card == *card_value).count())
                .collect_vec();

            number_of_each_card.sort();

            if number_of_each_card == vec![1, 1, 3] {
                return Self::ThreeOfAKind;
            }

            if number_of_each_card == vec![1, 2, 2] {
                return Self::TwoPair;
            }
        }

        if unique_cards.len() == 4 {
            let mut number_of_each_card = unique_cards
                .iter()
                .map(|card_value| cards.iter().filter(|card| **card == *card_value).count())
                .collect_vec();

            number_of_each_card.sort();

            if number_of_each_card == vec![1, 1, 1, 2] {
                return Self::OnePair;
            }
        }

        Self::HighCard
    }

    pub fn classify_hand_with_jokers(cards: &[char]) -> Self {
        let number_of_jokers = cards.iter().filter(|card| **card == 'J').count();

        let no_jokers_cards = cards
            .iter()
            .filter(|card| **card != 'J')
            .cloned()
            .collect_vec();

        CARD_ORDERING
            .chars()
            .combinations_with_replacement(number_of_jokers)
            .map(|mut combo| {
                combo.extend(no_jokers_cards.iter());

                HandClassification::classify_hand(&combo)
            })
            .min()
            .expect("Always at least one ordering")
    }
}

fn hand_vec_from_str(s: &str, part: Part) -> Vec<Hand> {
    s.lines()
        .map(|line| {
            if let Some((cards, bid)) = line.split_ascii_whitespace().collect_tuple() {
                Hand {
                    cards: cards.chars().collect_vec(),
                    bid: bid.parse().expect("bid always an integer"),
                    ordering: part,
                }
            } else {
                panic!("Invalid number of fields");
            }
        })
        .collect_vec()
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut hands = hand_vec_from_str(input, Part::Part1);

    hands.sort();

    Some(
        hands
            .iter()
            .rev()
            .enumerate()
            .fold(0, |acc, (rank, hand)| acc + (rank + 1) as u32 * hand.bid),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut hands = hand_vec_from_str(input, Part::Part2);

    hands.sort();

    Some(
        hands
            .iter()
            .rev()
            .enumerate()
            .fold(0, |acc, (rank, hand)| acc + (rank + 1) as u32 * hand.bid),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples/part1", DAY));
        assert_eq!(result, Some(6440));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples/part2", DAY));
        assert_eq!(result, Some(5905));
    }

    #[test]
    fn test_classify_hand() {
        assert_eq!(
            HandClassification::classify_hand(&"AAAAA".chars().collect_vec()),
            HandClassification::FiveOfAKind
        );
        assert_eq!(
            HandClassification::classify_hand(&"AA8AA".chars().collect_vec()),
            HandClassification::FourOfAKind
        );
        assert_eq!(
            HandClassification::classify_hand(&"23332".chars().collect_vec()),
            HandClassification::FullHouse
        );
        assert_eq!(
            HandClassification::classify_hand(&"TTT98".chars().collect_vec()),
            HandClassification::ThreeOfAKind
        );
        assert_eq!(
            HandClassification::classify_hand(&"23432".chars().collect_vec()),
            HandClassification::TwoPair
        );
        assert_eq!(
            HandClassification::classify_hand(&"A23A4".chars().collect_vec()),
            HandClassification::OnePair
        );
        assert_eq!(
            HandClassification::classify_hand(&"23456".chars().collect_vec()),
            HandClassification::HighCard
        );
        assert_eq!(
            HandClassification::classify_hand(&"32T3K".chars().collect_vec()),
            HandClassification::OnePair
        );
    }
}
