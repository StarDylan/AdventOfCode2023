use std::collections::HashSet;

use itertools::Itertools;

advent_of_code::solution!(7);

const CARD_ORDERING: &str = "AKQJT98765432J";

#[derive(Debug)]
struct HandPart1 {
    cards: Vec<char>,
    pub bid: u32
}

impl Ord for HandPart1 {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let classification_self = HandClassification::classify_hand(&self.cards);
        let other_classification = HandClassification::classify_hand(&other.cards);

        if classification_self == other_classification {
            // Do linear card search
            let numbered_cards = self.cards.iter().zip(other.cards.iter())
                .map(|(card_self, card_other)| 
                    (CARD_ORDERING.chars()
                        .position(|order_card| order_card == *card_self)
                        .expect("Always in CARD_ORDERING"),
                    CARD_ORDERING.chars()
                        .position(|order_card| order_card == *card_other)
                        .expect("Always in CARD_ORDERING")
                    )).collect_vec();
                
            
            for (self_card, other_card) in numbered_cards {
                match self_card.cmp(&other_card) {
                    std::cmp::Ordering::Less | std::cmp::Ordering::Greater => return self_card.cmp(&other_card),
                    std::cmp::Ordering::Equal => {},
                }
            }

            // Must be equal
            return std::cmp::Ordering::Equal;
        }
        classification_self.cmp(&other_classification)


    }
}

impl PartialEq for HandPart1 {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other).is_eq()
    }
}


impl PartialOrd for HandPart1 {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl Eq for HandPart1 {}

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
    pub fn classify_hand(cards: &Vec<char>) -> Self {
        let unique_cards: HashSet<char> = HashSet::from_iter(cards.iter().cloned());

        if unique_cards.len() == 1 {
            return Self::FiveOfAKind;
        } 
    
        
        if unique_cards.len() == 2 {
            let mut number_of_each_card = unique_cards.iter()
                .map(|card_value| 
                    cards.iter().filter(|card| **card == *card_value).count()
                )
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
            let mut number_of_each_card = unique_cards.iter()
                .map(|card_value| 
                    cards.iter().filter(|card| **card == *card_value).count()
                )
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
            let mut number_of_each_card = unique_cards.iter()
                .map(|card_value| 
                    cards.iter().filter(|card| **card == *card_value).count()
                )
                .collect_vec();

            number_of_each_card.sort();

            if number_of_each_card == vec![1, 1, 1, 2] {
                return Self::OnePair;
            }
        }

        Self::HighCard
    }

    pub fn classify_hand_with_jokers(cards: &Vec<char>) -> Self {
        let unique_cards: HashSet<char> = HashSet::from_iter(cards.iter().cloned());


        let cards_with_jokers_removed = cards.iter()
            .filter(|card| **card != 'J')
            .cloned();

        let unique_cards_no_jokers: HashSet<char> = HashSet::from_iter(cards_with_jokers_removed);


        if unique_cards_no_jokers.len() == 1 || unique_cards_no_jokers.len() == 0 {
            return Self::FiveOfAKind;
        } 
    
        
        if unique_cards.len() == 2 {
            let mut number_of_each_card = unique_cards_no_jokers.iter()
                .map(|card_value| 
                    ( card_value, cards.iter().filter(|card| **card == *card_value).count())
                )
                .collect_vec();

            number_of_each_card.sort();

            let most_card = number_of_each_card.first()
                .expect("Always at least one card");

            let best_case_for_four_of_a_kind = cards.iter()
                .map(|card| if *card == 'J' {*most_card.0} else {*card});

            let mut number_of_each_card_four_of_a_kind = best_case_for_four_of_a_kind
                .map(|card_value| 
                    cards.iter().filter(|card| **card == card_value).count()
                )
                .collect_vec();

            number_of_each_card.sort();

            if number_of_each_card_four_of_a_kind == vec![1, 4] {
                return Self::FourOfAKind;
            }


            

            if number_of_each_card == vec![2, 3] {
                return Self::FullHouse;
            }
        }

        if unique_cards.len() == 3 {
            let mut number_of_each_card = unique_cards.iter()
                .map(|card_value| 
                    cards.iter().filter(|card| **card == *card_value).count()
                )
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
            let mut number_of_each_card = unique_cards.iter()
                .map(|card_value| 
                    cards.iter().filter(|card| **card == *card_value).count()
                )
                .collect_vec();

            number_of_each_card.sort();

            if number_of_each_card == vec![1, 1, 1, 2] {
                return Self::OnePair;
            }
        }

        Self::HighCard
    }
}

fn hand_vec_from_str(s: &str) -> Vec<HandPart1> {
    s.lines()
        .map(|line| {
            if let Some((cards, bid)) = line.split_ascii_whitespace().collect_tuple() {
                HandPart1 {
                    cards: cards.chars().collect_vec(),
                    bid: bid.parse().expect("bid always an integer")
                }

            } else {
                panic!("Invalid number of fields");
            }
        }).collect_vec()
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut hands = hand_vec_from_str(input);

    hands.sort();

    Some(hands.iter().rev().enumerate().fold(0, |acc, (rank, hand)| acc + (rank + 1 ) as u32 * hand.bid))
}

pub fn part_two(input: &str) -> Option<u32> {
    None
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
        assert_eq!(result, None);
    }

    #[test]
    fn test_classify_hand() {
        assert_eq!(HandClassification::classify_hand(&"AAAAA".chars().collect_vec()), HandClassification::FiveOfAKind);
        assert_eq!(HandClassification::classify_hand(&"AA8AA".chars().collect_vec()), HandClassification::FourOfAKind);
        assert_eq!(HandClassification::classify_hand(&"23332".chars().collect_vec()), HandClassification::FullHouse);
        assert_eq!(HandClassification::classify_hand(&"TTT98".chars().collect_vec()), HandClassification::ThreeOfAKind);
        assert_eq!(HandClassification::classify_hand(&"23432".chars().collect_vec()), HandClassification::TwoPair);
        assert_eq!(HandClassification::classify_hand(&"A23A4".chars().collect_vec()), HandClassification::OnePair);
        assert_eq!(HandClassification::classify_hand(&"23456".chars().collect_vec()), HandClassification::HighCard);
        assert_eq!(HandClassification::classify_hand(&"32T3K".chars().collect_vec()), HandClassification::OnePair);
    }
}
