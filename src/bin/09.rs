#![feature(iter_map_windows)]

advent_of_code::solution!(9);

pub fn part_one(input: &str) -> Option<i64> {
    let sequences: Vec<Vec<i64>> = input
        .lines()
        .map(|s| {
            s.split_ascii_whitespace()
                .map(|s| s.parse().expect("strings should be numbers"))
                .collect()
        })
        .collect();

    let extrapolations: Vec<i64> = sequences.iter().map(|seq| extraplotate(seq)).collect();

    // dbg!(&extrapolations);

    Some(extrapolations.iter().sum())
}

fn compute_differences(seq: &Vec<i64>) -> Vec<i64> {
    seq.iter().map_windows(|[x, y]| *y - *x).collect()
}

fn extraplotate(seq: &Vec<i64>) -> i64 {
    let mut differences: Vec<Vec<i64>> = Vec::new();

    differences.push(compute_differences(seq));

    while !differences
        .last()
        .expect("should have a last")
        .iter()
        .all(|x| *x == 0)
    {
        differences.push(compute_differences(
            differences.last().expect("should have a last"),
        ));
    }

    differences.last_mut().expect("should have a last").push(0);

    for i in (0..differences.len() - 1).rev() {
        let prev_diff = *differences[i + 1].last().unwrap();
        let cur_diff = *differences[i].last().unwrap();

        (&mut differences[i]).push(cur_diff + prev_diff);
    }

    let delta_next = differences[0].last().expect("should be a sequence");

    seq.last().expect("should be a sequence") + delta_next
}

pub fn part_two(input: &str) -> Option<i64> {
    let sequences_iter = input.lines().map(|s| {
        s.split_ascii_whitespace()
            .map(|s| s.parse().expect("strings should be numbers"))
    });

    // Reverse sequence
    let sequences: Vec<Vec<i64>> = sequences_iter.map(|seq| seq.rev().collect()).collect();

    let extrapolations: Vec<i64> = sequences.iter().map(|seq| extraplotate(seq)).collect();

    Some(extrapolations.iter().sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(114));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }
}
