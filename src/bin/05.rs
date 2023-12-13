use std::cmp::{max, min};

advent_of_code::solution!(5);

pub fn part_one(input: &str) -> Option<u32> {
    let lines: Vec<&str> = input.lines().collect();

    let seeds = lines[0].split(":").last().expect("invalid seeds");
    let seeds: Vec<u64> = seeds
        .split_ascii_whitespace()
        .map(|s| s.parse().expect("invalid seed u32s"))
        .collect();

    dbg!(&seeds);

    let mut mappings: Vec<Vec<(u64, u64, u64)>> = Vec::new();

    for line in &lines[1..] {
        if line.contains("map:") {
            mappings.push(Vec::new());
        } else if line.contains(|c: char| c.is_ascii_digit()) {
            let current_mapping = mappings
                .last_mut()
                .expect("expected to have already seen a map: string");

            let numbers: Vec<u64> = line
                .split_ascii_whitespace()
                .map(|s| s.parse().expect("invalid u32"))
                .collect();

            assert!(numbers.len() == 3);

            current_mapping.push((numbers[0], numbers[1], numbers[2]));
        }
    }

    let mut mapped_values = Vec::new();

    for ref seed in seeds {
        let mut current_value: u64 = *seed;

        for mapping in &mappings {
            current_value = map_value(current_value, &mapping);
        }

        mapped_values.push(current_value);
    }

    Some(mapped_values.into_iter().min().expect("failed to find min") as u32)
}

fn map_value(current_value: u64, mapping: &Vec<(u64, u64, u64)>) -> u64 {
    for (dst, src, rl) in mapping {
        if (*src..(*src + *rl)).contains(&current_value) {
            return current_value - src + dst;
        }
    }

    return current_value;
}

pub fn part_two(input: &str) -> Option<u32> {
    let lines: Vec<&str> = input.lines().collect();

    let parsed_seeds = lines[0].split(":").last().expect("invalid seeds");
    let parsed_seeds: Vec<u64> = parsed_seeds
        .split_ascii_whitespace()
        .map(|s| s.parse().expect("invalid seed u32s"))
        .collect();

    let mut seed_ranges: Vec<(u64, u64)> = Vec::new();

    for i in 0..(parsed_seeds.len() / 2) {
        seed_ranges.push((parsed_seeds[i * 2], parsed_seeds[i * 2 + 1]));
    }

    let mut mappings: Vec<Vec<(u64, u64, u64)>> = Vec::new();

    for line in &lines[1..] {
        if line.contains("map:") {
            mappings.push(Vec::new());
        } else if line.contains(|c: char| c.is_ascii_digit()) {
            let current_mapping = mappings
                .last_mut()
                .expect("expected to have already seen a map: string");

            let numbers: Vec<u64> = line
                .split_ascii_whitespace()
                .map(|s| s.parse().expect("invalid u32"))
                .collect();

            assert!(numbers.len() == 3);

            current_mapping.push((numbers[0], numbers[1], numbers[2]));
        }
    }

    let mut mapped_values = Vec::new();

    for ref seed in &seed_ranges {
        let mut current_ranges: Vec<(u64, u64)> = vec![**seed];

        for mapping in &mappings {
            current_ranges = map_ranges(current_ranges, &mapping);
        }

        mapped_values.push(get_lowest_from_ranges(current_ranges));
    }

    Some(mapped_values.into_iter().min().expect("failed to find min") as u32)
}

fn get_lowest_from_ranges(current_ranges: Vec<(u64, u64)>) -> u32 {
    // dbg!(&current_ranges);

    return current_ranges
        .into_iter()
        .min_by(|(x, _), (y, _)| x.cmp(y))
        .expect("could not find min")
        .0 as u32;
}

fn map_ranges(current_ranges: Vec<(u64, u64)>, mapping: &[(u64, u64, u64)]) -> Vec<(u64, u64)> {
    let mut queue = Vec::from_iter(current_ranges.clone());

    let mut mapped_ranges = Vec::new();

    while let Some((start, len)) = queue.pop() {
        let mut found_at_least_one_mapping = false;

        for (dst, src, rl) in mapping {
            if start < src + rl && start + len > *src {
                // defined as start, end ranges as [,) range
                let range_before = (start, *src);
                let range_overlap = (max(*src, start), min(*src + *rl, start + len));
                let range_after = (*src + *rl, start + len);

                if range_before.1 > range_before.0 {
                    queue.push((range_before.0, range_before.1 - range_before.0));
                }

                if range_after.1 > range_after.0 {
                    queue.push((range_after.0, range_after.1 - range_after.0));
                }

                assert!(range_overlap.1 > range_overlap.0);

                mapped_ranges.push((
                    range_overlap.0 - *src + *dst,
                    range_overlap.1 - range_overlap.0,
                ));

                found_at_least_one_mapping = true;
            }
        }

        if found_at_least_one_mapping == false {
            mapped_ranges.push((start, len));
        }
    }

    mapped_ranges
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result: Option<u32> = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(35));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(46));
    }
}
