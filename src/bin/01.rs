advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u32> {
    let value_mapping = make_value_mapping_part_1();


    let mut sum_of_values = 0;

    for line in input.lines() {
        let digits: Vec<u32> = recover_digits(line, &value_mapping);            

        match recover_calibration_value(&digits) {
            Some(value) => {
                // println!("{} -> {:?}: {}", line_string, digits, value);

                sum_of_values += value;
            }
            None => {
                println!("{} -> {:?}: failed to recover value", &line, digits);
            }
        }
    }

    Some(sum_of_values)
}

pub fn part_two(input: &str) -> Option<u32> {
    let value_mapping = make_value_mapping_part_2();

    let mut sum_of_values = 0;

    for line in input.lines() {
        let digits: Vec<u32> = recover_digits(line, &value_mapping);            

        match recover_calibration_value(&digits) {
            Some(value) => {
                sum_of_values += value;
            }
            None => {
                println!("{} -> {:?}: failed to recover value", &line, digits);
            }
        }
    }

    Some(sum_of_values)
}

fn make_value_mapping_part_1() -> Vec<(String, u32)> {
    let digit_value_mapping: Vec<(String, u32)> = (1..=9).map(|v| (v.to_string().to_owned(), v as u32)).collect();

    digit_value_mapping
}

const NUMBERS: [&str; 9] = ["one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];

fn make_value_mapping_part_2() -> Vec<(String, u32)> {

    let mut word_value_mapping: Vec<(String, u32)> = NUMBERS
        .iter()
        .enumerate()
        .map(|(index, &item)| (item.to_owned(), (index + 1) as u32))
        .collect(); 
    
    let mut digit_value_mapping: Vec<(String, u32)> = (1..=9).map(|v| (v.to_string().to_owned(), v as u32)).collect();

    assert_eq!(digit_value_mapping.len(), 9, "There should be nine digits, but digit_value_mapping={:?}", digit_value_mapping);
    
    word_value_mapping.append(&mut digit_value_mapping);

    word_value_mapping
}

fn recover_digits(line: &str, value_mapping: &Vec<(String, u32)>) -> Vec<u32> {

    let mut found: Vec<(usize, u32)> = vec![];

    for (pattern, value) in value_mapping {

        let mut matches: Vec<(usize, u32)> = line.match_indices(pattern).map(|(i, _)| (i, value.to_owned())).collect();

        found.append(&mut matches);
    }

    found.sort_by(|(a, _), (b, _)| a.cmp(b));

    found.iter().map(|(_, v)| v.to_owned()).collect()
}

fn recover_calibration_value(digits: &Vec<u32>) -> Option<u32> {

    return match (digits.first(), digits.last()) {
        (Some(first), Some(last)) => Some(10 * first + last),
        _ => None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(142));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part ("examples", DAY, 2));
        assert_eq!(result, Some(281));
    }

    #[test]
    fn test_spelled_digits_into_digits() {
        let input = "2fivejrnpfbfive3grhdcngfvkxqrl5";

        assert_eq!(recover_digits(&input.to_owned(), &make_value_mapping_part_2()), vec![2,5,5,3,5]);
    }

    #[test]
    fn test_spelled_digits_into_digits_not_zero() {
        let input = "zero2fivejrnpfbfive3grhdcngfvkxqrl5";

        assert_eq!(recover_digits(&input.to_owned(), &make_value_mapping_part_2()), vec![2,5,5,3,5]);
    }


    #[test]
    fn test_spelled_digits_into_digits_replaced_first_occurence() {
        let input = "eightwothree".to_owned();

        assert_eq!(recover_digits(&input.to_owned(), &make_value_mapping_part_2()), vec![8, 2, 3]);
    }
}
