use itertools::Itertools;
use joinery::JoinableIterator;

advent_of_code::solution!(6);

#[derive(Debug)]
struct Race {
    time: u64,
    distance: u64
}

impl From<(u64, u64)> for Race {
    fn from(value: (u64, u64)) -> Self {
        Race { time: value.0, distance: value.1 }
    }
}

fn races_from_str(s: &str) -> Vec<Race> {

    let mut parts = s.lines();

    let times = parts.next().expect("awlays a time line")
        .split_ascii_whitespace().skip(1);

    let distances: std::iter::Skip<std::str::SplitAsciiWhitespace<'_>> = parts.next().expect("awlays a dist line")
        .split_ascii_whitespace().skip(1);

    times.zip(distances)
        .map(|(time, dist)| 
            (
                time.parse().expect("Time always a number"), 
                dist.parse().expect("Dist always a number")
            ))
        .map(Into::<Race>::into)
        .collect_vec()
}

fn race_from_str_no_gaps(s: &str) -> Race {
    let mut parts = s.lines();

    let time = parts.next().expect("awlays a time line")
        .split_ascii_whitespace().skip(1).join_concat().to_string();

    let distance = parts.next().expect("awlays a dist line")
        .split_ascii_whitespace().skip(1).join_concat().to_string();

    Race {
        time: time.parse().expect("Time always a number"),
        distance: distance.parse().expect("Dist always a number")
    }
}

fn num_ways_to_beat_race(race: &Race) -> u64 {
    (0..=race.time)
        .map(|button_hold_time| {
            button_hold_time * (race.time - button_hold_time)
        })
        .filter(|traveled_distance| *traveled_distance > race.distance)
        .count() as u64
}

pub fn part_one(input: &str) -> Option<u64> {
    let races = races_from_str(input);

    Some(races
        .iter()
        .map(num_ways_to_beat_race)
        .product())

}

pub fn part_two(input: &str) -> Option<u64> {
    let race = race_from_str_no_gaps(input);

    Some(num_ways_to_beat_race(&race))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples/part1", DAY));
        assert_eq!(result, Some(288));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples/part2", DAY));
        assert_eq!(result, Some(71503));
    }
}
