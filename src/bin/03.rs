use std::cmp::{max, min};

use regex::Regex;

advent_of_code::solution!(3);

pub fn part_one(input: &str) -> Option<u32> {

    let lines = input.lines().collect();

    let parts = find_parts(&lines);

    let found_numbers = parts.into_iter().map(|(value, _)| value);

    Some(found_numbers.into_iter().sum())
}

fn find_parts(lines: &Vec<&str>) -> Vec<(u32, (i32, (i32, i32)))> {

    let find_numbers = Regex::new(r"\d+").expect("Invalid regex");

    let mut parts: Vec<(u32, (i32, (i32, i32)))> = Vec::new();

    for (i, line) in lines.iter().enumerate() {
        let i = i as i32;
        for (j_start, substring) in line.match_indices(&find_numbers) {
            let j_start = j_start;

            let j_end = j_start + substring.chars().count();

            let scan_start: usize = max(0, (j_start as i32) - 1) as usize;
            let scan_end: usize = min(line.chars().count(), j_end + 1);

            let mut is_part = false;

            let search_range = ((i-1)..=(i+1)).filter(|i| i >= &0).map(|i| i as usize);

            for neighbor_i in search_range {
                if let Some(neighbor_line) = lines.get(neighbor_i) {
                    if neighbor_line[scan_start..scan_end].contains(|c: char| !(c.is_numeric() || c == '.')) {
                        is_part = true;
                    }
                }
            }

            if is_part {

                let value: u32 = substring.parse().expect("Expected to parse the number");

                parts.push((value, (i as i32, (j_start as i32, j_end as i32))));
            }
        }
    }
    parts
}

pub fn part_two(input: &str) -> Option<u32> {
    let lines: Vec<&str> = input.lines().collect();
    
    let parts = find_parts(&lines);

    let mut gear_ratios = Vec::new();

    for (star_i, line) in lines.iter().enumerate() {
        let star_i = star_i as i32;

        for (star_j, _) in line.match_indices('*') {
            let star_j = star_j as i32;
            // let search_range = ((i-1)..=(i+1)).filter(|i| i >= &0).map(|i| i as usize);

            let mut star_values = Vec::new();

            println!("checking star: {},{}", star_i, star_j);

            for (part_value, (part_i, (part_j_start, part_j_end))) in &parts {
                if star_i + 1 >= *part_i && star_i - 1 <= *part_i {
                    if star_j >= *part_j_start - 1 && star_j <= *part_j_end {
                        star_values.push(part_value);
                    }
                }
            }

            dbg!(&star_values);

            if star_values.len() == 2 {
                let gear_ratio: u32 = star_values.iter().copied().product();

                gear_ratios.push(gear_ratio);

                println!("Found gear! {:?} -> {}", &star_values, gear_ratio);
            }
        }
    }

    Some(gear_ratios.into_iter().sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));

        assert_eq!(result, Some(4361));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(467835));
    }
}
