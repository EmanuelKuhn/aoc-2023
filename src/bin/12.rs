use std::{option, vec, collections::HashMap};

advent_of_code::solution!(12);

fn recursive_options(conditions: &str, cgods: &Vec<usize>) -> usize {
    let has_unkown = conditions.contains('?');

    match has_unkown {
        true => {
            let partial = recursive_options(conditions.replacen('?', ".", 1).as_str(), cgods)
            + recursive_options(conditions.replacen('?', "#", 1).as_str(), cgods);

            return partial
        }
        false => {
            match matches_cgods(conditions, cgods) {
                true => return 1,
                false => {return 0},
            } 
        }
    }
}

fn matches_cgods(conditions: &str, cgods: &[usize]) -> bool {
    let group_sizes: Vec<usize> = conditions.split('.').map(|s| s.len()).filter(|&l| l > 0).collect::<Vec<usize>>();
    
    let equals = cgods.eq(&group_sizes);

    // dbg!(conditions, group_sizes, equals, cgods);

    equals
}


fn process_line(input: &str) -> usize {
    let (conditions, cgods) = input.split_once(' ').expect("expected space char");

    // dbg!(&conditions);
    // dbg!(&cgods);

    let cgods: Vec<usize> = cgods.split(',').map(|s| s.parse().expect("should be number")).collect();

    // recursive_options(conditions, &cgods)

    let mut sol = MemoizedSolution::new();

    sol.count(conditions, cgods, false)
}

pub fn part_one(input: &str) -> Option<usize> {
    let mut result = 0;

    for line in input.lines() {
        // dbg!(&line);

        result += process_line(line);
    }

    Some(result)
}

struct MemoizedSolution<K> {
    cache: HashMap<K, usize>
}

impl<'a> MemoizedSolution<(&'a str, Vec<usize>, bool)> {

    fn new() -> Self {
        Self {cache: HashMap::new()}
    }

    fn count(&mut self, springs: &'a str, groups: Vec<usize>, in_group: bool) -> usize {

        let key = (springs, groups.clone(), in_group);

        if let Some(&result) = self.cache.get(&key) {
            return result;
        }

        if springs.is_empty() {
            match groups.iter().filter(|&&g| g > 0).count() == 0 {
                true => return 1,
                false => return 0,
            }
        }

        let mut groups = groups.clone();

        let (current_spring, tail) = springs.split_at(1);

        let result = match current_spring {
            "." => {
                if in_group && groups[0] != 0 {
                    return 0;
                } else if in_group {
                    // assert!(groups[0] == 0);
                    groups.remove(0);   
                }

                self.count(tail, groups, false)
            },
            "#" => {
                match groups.len() == 0 || groups[0] == 0 {
                    true => 0,
                    false => {
                        groups[0] -= 1;
                        self.count(tail, groups, true)
                    }
                }
            },
            "?" => {
                if groups.len() == 0 {
                    return self.count(tail, groups, false)
                }

                match groups[0] {
                    0 => {
                        groups.remove(0);
                        self.count(tail, groups, false)
                    },
                    n => {
                        let mut groups_dec = groups.clone();
                        groups_dec[0] -= 1;

                        let count_in_group = self.count(tail, groups_dec, true);

                        match in_group {
                            true => count_in_group,
                            false => count_in_group + self.count(tail, groups, false)
                        }
                    }
                }
            },
            _ => panic!("Invalid spring state")
        };

        self.cache.insert(key, result);

        result
    }
}


fn process_line2(line: &str) -> usize {
    let (conditions, cgods) = line.split_once(' ').expect("expected space char");

    let conditions = vec![conditions, conditions, conditions, conditions, conditions].join("?");
    let cgods = vec![cgods, cgods, cgods, cgods, cgods].join(",");
    
    let cgods: Vec<usize> = cgods.split(',').map(|s| s.parse().expect("should be number")).collect();

    let mut sol = MemoizedSolution::new();

    sol.count(&conditions, cgods, false)
}

pub fn part_two(input: &str) -> Option<usize> {
    let mut result = 0;

    for line in input.lines() {
        // dbg!(&line);

        result += process_line2(line);
    }

    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(21));
    }

    #[test]
    fn test_part_two() {
        let result: Option<usize> = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(525152));
    }
}
