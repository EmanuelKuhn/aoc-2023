use std::iter::zip;

advent_of_code::solution!(6);

pub fn part_one(input: &str) -> Option<u32> {
    let lines: Vec<&str> = input.lines().collect();

    let times: Vec<u64> = lines[0]
        .split(":")
        .last()
        .expect("times : missing")
        .split_whitespace()
        .map(|s: &str| s.parse().expect("msg"))
        .collect();
    let distances: Vec<u64> = lines[1]
        .split(":")
        .last()
        .expect("distance : missing")
        .split_whitespace()
        .map(|s: &str| s.parse().expect("msg"))
        .collect();

    // dbg!(&times);
    // dbg!(&distances);

    let mut win_options_per_race: Vec<u32> = Vec::new();

    for (time, distance) in zip(times, distances) {
        let win_options = compute_win_options(time, distance);

        win_options_per_race.push(win_options);
    }

    // dbg!(&win_options_per_race);

    Some(win_options_per_race.into_iter().product())
}

fn compute_win_options(time: u64, distance: u64) -> u32 {
    let min_distance_to_win = distance + 1;

    let discriminant = ((time.pow(2) - 4 * (min_distance_to_win)) as f64).sqrt();

    let min_hold_time: f64 = (time as f64 - discriminant) / 2.;
    let max_hold_time: f64 = (time as f64 + discriminant) / 2.;

    let min_hold_time = min_hold_time.ceil() as u32;
    let max_hold_time = max_hold_time.floor() as u32;

    let amount_of_options = max_hold_time - min_hold_time + 1;

    amount_of_options
}

pub fn part_two(input: &str) -> Option<u32> {
    let lines: Vec<&str> = input.lines().collect();

    let mut times = lines[0]
        .split(":")
        .last()
        .expect("times : missing")
        .to_string();
    times.retain(|c| !c.is_whitespace());

    let time: u64 = times.parse().expect("could not parse time");

    let mut distances = lines[1]
        .split(":")
        .last()
        .expect("distances : missing")
        .to_string();
    distances.retain(|c| !c.is_whitespace());

    let distance: u64 = distances.parse().expect("could not parse distance");

    // dbg!(time, distance);

    Some(compute_win_options(time, distance))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(288));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(71503));
    }
}
