use itertools::Itertools;

advent_of_code::solution!(9);

fn time_data_from_file(s: &str) -> Vec<Vec<i64>> {
    s.trim()
        .lines()
        .map(|line| {
            line.split_ascii_whitespace()
                .map(|num| num.parse().expect("Always a num"))
                .collect_vec()
        })
        .collect_vec()
}

fn extrapolate(data: &[i64]) -> i64 {
    let mut num_layers = 0;

    let mut current_data = data.to_owned();

    let mut first_points = Vec::new();

    while !current_data.iter().all(|num| *num == 0) {
        first_points.push(*current_data.first().expect("always a first"));

        current_data = current_data.windows(2).map(|w| w[1] - w[0]).collect_vec();
        num_layers += 1;
    }

    // We have all zeros, lets add a zero
    current_data.push(0);

    // Now the reverse
    for layer in 0..num_layers {
        let mut next_layer = Vec::new();
        next_layer.push(
            *first_points
                .get(first_points.len() - 1 - layer)
                .expect("Always a first point for each layer"),
        );
        for idx in 1..current_data.len() + 1 {
            let base = *next_layer
                .get(idx - 1)
                .expect("Always a valid idx to get, we just added it");

            let diff = *current_data
                .get(idx - 1)
                .expect("Always a valid idx to get");

            next_layer.push(base + diff);
        }

        current_data = next_layer;
    }

    *current_data.last().expect("Always at least one datapoint")
}

pub fn part_one(input: &str) -> Option<i64> {
    let data = time_data_from_file(input);

    Some(data.iter().map(|v| extrapolate(v)).sum())
}

pub fn part_two(input: &str) -> Option<i64> {
    let mut data = time_data_from_file(input);

    // We need to back-extrapolate, so lets reverse all the sequences
    for vec in &mut data {
        vec.reverse();
    }

    Some(data.iter().map(|v| extrapolate(v)).sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples/part1", DAY));
        assert_eq!(result, Some(114));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples/part2", DAY));
        assert_eq!(result, None);
    }
}
