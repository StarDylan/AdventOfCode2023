use itertools::Itertools;
advent_of_code::solution!(5);

#[derive(Debug, Clone)]
struct Mapping {
    dest_start: u64,
    source_start: u64,
    range_len: u64,
}

impl Mapping {
    pub fn reverse(self) -> Self {
        Mapping {
            dest_start: self.source_start,
            source_start: self.dest_start,
            range_len: self.range_len,
        }
    }
}

struct SeedRange {
    start: u64,
    length: u64,
}

impl SeedRange {
    pub fn is_in_range(&self, value: u64) -> bool {
        self.start <= value && value < self.start + self.length
    }
}

#[derive(Debug, Clone)]
struct MapFunction {
    funcs: Vec<Mapping>,
}

impl Mapping {
    pub fn from_line(s: &str) -> Mapping {
        let mut parts = s.split_ascii_whitespace();

        Mapping {
            dest_start: parts
                .next()
                .expect("always exists")
                .parse()
                .expect("Always u32"),
            source_start: parts
                .next()
                .expect("always exists")
                .parse()
                .expect("Always u32"),
            range_len: parts
                .next()
                .expect("always exists")
                .parse()
                .expect("Always u32"),
        }
    }
}

impl MapFunction {
    pub fn apply_function(&self, input: u64) -> u64 {
        let maybe_relevant_func = self.funcs.iter().find(|mapping| {
            mapping.source_start <= input && input < mapping.source_start + mapping.range_len
        });

        match maybe_relevant_func {
            Some(relevant_func) => {
                let offset = input - relevant_func.source_start;
                relevant_func.dest_start + offset
            }
            None => input,
        }
    }

    pub fn reverse(self) -> Self {
        Self {
            funcs: self.funcs.into_iter().map(|f| f.reverse()).collect_vec(),
        }
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    let initial_seeds = input
        .lines()
        .next()
        .expect("always has a first line")
        .split(": ")
        .nth(1)
        .expect("Always a num list part")
        .split_ascii_whitespace()
        .map(|s| s.parse::<u64>().expect("Every number is valid u32"))
        .collect_vec();

    let maps = input
        .split("\n\n")
        .skip(1) // Skip initial seed
        .map(|map_func| {
            let funcs = map_func
                .lines()
                .skip(1) // Skip title
                .map(Mapping::from_line)
                .collect_vec();
            MapFunction { funcs }
        })
        .collect_vec();

    let final_ids = maps.iter().fold(initial_seeds, |ids, mapping_func| {
        let ids = ids
            .iter()
            .map(|id| mapping_func.apply_function(*id))
            .collect_vec();
        ids
    });

    let lowest_location = final_ids
        .iter()
        .min()
        .expect("Always at least one initial seed");

    Some(*lowest_location)
}

pub fn part_two(input: &str) -> Option<u64> {
    let initial_seeds = input
        .lines()
        .next()
        .expect("always has a first line")
        .split(": ")
        .nth(1)
        .expect("Always a num list part")
        .split_ascii_whitespace()
        .chunks(2)
        .into_iter()
        .map(|mut s| {
            let start = s
                .next()
                .expect("Always a start num")
                .parse()
                .expect("Always num");
            let length: u64 = s
                .next()
                .expect("Always a length num")
                .parse()
                .expect("Always num");

            SeedRange { start, length }
        })
        .collect_vec();

    let mut maps = input
        .split("\n\n")
        .skip(1) // Skip initial seed
        .map(|map_func| {
            let funcs = map_func
                .lines()
                .skip(1) // Skip title
                .map(Mapping::from_line)
                .collect_vec();
            MapFunction { funcs }
        })
        .map(|map_func| map_func.reverse())
        .collect_vec();

    maps.reverse();

    let lowest_ending_that_exists = (0..u64::MAX)
        .find(|ending_id| {
            let start_id = maps.iter().fold(*ending_id, |id, mapping_func| {
                mapping_func.apply_function(id)
            });
            initial_seeds
                .iter()
                .any(|initial| initial.is_in_range(start_id))
        })
        .expect("Always one valid num");

    Some(lowest_ending_that_exists)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples/part1", DAY));
        assert_eq!(result, Some(35));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples/part2", DAY));
        assert_eq!(result, Some(46));
    }
}
