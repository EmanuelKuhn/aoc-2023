use std::{collections::{HashMap, HashSet}, mem::swap};

advent_of_code::solution!(8);

pub fn part_one(input: &str) -> Option<u64> {
    let (instructions, states) = parse_input(input)?;

    let current_state = "AAA";

    // return None;

    Some(run_chain(current_state, &states, &instructions, |s| s == "ZZZ"))
}

fn parse_input(input: &str) -> Option<(Vec<char>, HashMap<&str, (&str, &str)>)> {
    let mut lines_it = input.lines();
    let instructions: Vec<char> = lines_it.next()?.chars().collect();
    let mut states = HashMap::new();
    for line in lines_it {
        if !line.contains('=') {
            continue;
        }

        let name = line.split("=").next().unwrap().trim();

        let tuple_string = line.split('(').last().expect("expected (").trim_matches(')');

        let tuple: Vec<_> = tuple_string.split(',').map(|s| s.trim()).collect();

        assert!(tuple.len() == 2);

        states.insert(name, (tuple[0], tuple[1]));
    }
    Some((instructions, states))
}

fn step_state<'a>(instruction: char, current_state: &str, states: &HashMap<&str, (&'a str, &'a str)>) -> &'a str {
    match instruction {
        'L' => return states[current_state].0,
        'R' => return states[current_state].1,
        _ => unreachable!()
    }
}

fn run_chain(starting_state: &str, states_map: &HashMap<&str, (&str, &str)>, instructions: &Vec<char>, finish_condition: fn(&str) -> bool) -> u64 {
    let mut current_state = starting_state;

    let mut steps = 0;
    
    let mut seen = HashSet::new();

    while !finish_condition(&current_state) {
        let instruction = instructions[steps % instructions.len()];

        seen.insert(current_state);

        current_state = step_state(instruction, current_state, &states_map);

        // dbg!(current_state);
        
        steps += 1;
    }

    steps as u64
}

fn finish_condition_ends_with_z(current_state: &str) -> bool {
    current_state.ends_with('Z')
}

fn greatest_common_divisor(mut a: u64, mut b:u64) -> u64 {
    assert!(a > 0 && b >0);

    while b != 0 {
        if a > b {
            swap(&mut a, &mut b);
        }

        b = b % a;
    }

    a
}

fn least_common_multiple(a: u64, b: u64) -> u64 {
    assert!(a > 0 && b > 0);

    a * (b / greatest_common_divisor(a, b))
}

pub fn part_two(input: &str) -> Option<u64> {
    let (instructions, states_map) = parse_input(input)?;

    let start_states: Vec<&str> = 
    states_map.keys().into_iter().filter(|s|  s.ends_with("A")).map(|s| *s).collect();

    let state_steps: Vec<_> = start_states.into_iter()
        .map(|s| run_chain(s, &states_map, &instructions, finish_condition_ends_with_z)).collect();

    // dbg!(&state_steps);

    let lcm = state_steps.into_iter().reduce(|acc, e| if acc > 0 {least_common_multiple(acc, e)} else {e}).expect("state_steps should have items");

    Some(lcm)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part("examples", DAY, 2));
        assert_eq!(result, Some(6));
    }

    #[test]
    fn test_gcd() {
        let result = greatest_common_divisor(54, 24);
        assert_eq!(result, 6);
    }
}
