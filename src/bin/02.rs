use std::cmp::max;

use itertools::Itertools;

advent_of_code::solution!(2);

struct Round {
    red: u32,
    green: u32,
    blue: u32,
}

impl Round {
    fn zeros() -> Round {
        Self {
            red: 0,
            green: 0,
            blue: 0,
        }
    }
}

struct Game {
    id: u32,
    rounds: Vec<Round>,
}

impl Game {
    fn from_str(s: &str) -> Self {
        let mut parts = s.split(':');
        let game = parts
            .next()
            .expect("Always a game label")
            .split_ascii_whitespace()
            .skip(1)
            .next()
            .expect("Always a game id")
            .parse()
            .expect("Always an integer");

        let rounds = parts.next().expect("Always rounds").trim().split("; ");

        let rounds = rounds
            .map(|round| {
                let mut red = 0;
                let mut green = 0;
                let mut blue = 0;

                round.split(", ").for_each(|cube_combo| {
                    let mut round_parts = cube_combo.split_ascii_whitespace();

                    let num: u32 = round_parts
                        .next()
                        .expect("Always a number")
                        .parse()
                        .expect("Always integer");
                    let color = round_parts.next().expect("Always a color");

                    match color {
                        "red" => red += num,
                        "green" => green += num,
                        "blue" => blue += num,
                        _ => {
                            panic!("Invalid color {color}!")
                        }
                    }
                });

                Round { red, green, blue }
            })
            .collect_vec();

        Self { id: game, rounds }
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let games = input.lines().map(|line| Game::from_str(line));

    Some(
        games
            .map(|game| {
                let possible = game
                    .rounds
                    .iter()
                    .find(|round| round.red > 12 || round.green > 13 || round.blue > 14)
                    .is_some();

                if !possible {
                    game.id
                } else {
                    0
                }
            })
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    let games = input.lines().map(|line| Game::from_str(line));

    Some(
        games
            .map(|game| {
                let min_game_config =
                    game.rounds
                        .iter()
                        .fold(Round::zeros(), |min_cubes, round| Round {
                            red: max(min_cubes.red, round.red),
                            green: max(min_cubes.green, round.green),
                            blue: max(min_cubes.blue, round.blue),
                        });

                min_game_config.red * min_game_config.green * min_game_config.blue
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
        assert_eq!(result, Some(8));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples/part2", DAY));
        assert_eq!(result, Some(2286));
    }
}
