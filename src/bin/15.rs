use std::str::FromStr;

advent_of_code::solution!(15);

pub fn part_one(input: &str) -> Option<u32> {
    let steps: Vec<&str> = input.trim_end().split(',').collect();
    
    Some(steps.iter().map(assci_hash).sum())

}

fn assci_hash(text: &&str) -> u32 {
    
    assert!(text.is_ascii());
    text.chars().map(|c| c as u8 as u32).fold(0, |acc, c| ((acc + c) * 17) % 256)
}

pub fn part_two(input: &str) -> Option<u32> {
    let steps: Vec<&str> = input.trim_end().split(',').collect();

    let mut boxes: Vec<Vec<(&str, u32)>> = vec![Vec::new(); 256];

    // dbg!(&steps);

    for step in steps {
        let step: Step = step.into();

        let box_index = assci_hash(&step.label) as usize;

        let the_box = &mut boxes[box_index];

        match step.op {
            Operator::Equals(focal_length) => {
                let lens = the_box.iter_mut().find(|(label, _)| label == &step.label);

                match lens {
                    Some(lens) => lens.1 = focal_length,
                    None => the_box.push((step.label, focal_length))
                }
            },
            Operator::Dash => {
                let lens_index = the_box.iter().position(|(label, _)| label == &step.label);

                if let Some(index) = lens_index {
                    the_box.remove(index);
                }
            },
        }

        // dbg!(boxes.iter().enumerate().filter(|(i, box_)| !box_.is_empty()).collect::<Vec<(_, &Vec<(_, _)>)>>());
    }

    Some(boxes.iter().enumerate().map({|(box_index, lenses)|
        lenses.iter().enumerate().map({|(lens_index, &(_, focal_length))|
            (box_index as u32 + 1) * (lens_index as u32 + 1) * focal_length
        }).sum::<u32>()
    }).sum())
}

enum Operator {
    Equals(u32),
    Dash
}


#[derive(Debug)]
enum ParseStepError {
    OperatorParseError(String)
}

impl FromStr for Operator {
    type Err = ParseStepError;
    
    fn from_str(value: &str) -> Result<Self, ParseStepError> {
        let (op, focal_length) = value.split_at(1);

        match op {
            "-" => Ok(Self::Dash),
            "=" => {
                let focal_length = focal_length.parse().unwrap_or_else(|_| panic!("should be number: {} ({})", focal_length, value));

                Ok(Self::Equals(focal_length))
            }
            _ => Err(ParseStepError::OperatorParseError(String::from(op)))
        }
    }
}

struct Step<'a> {
    label: &'a str,
    op: Operator
}

impl<'a> From<&'a str> for Step<'a> {

    fn from(step: &'a str) -> Self {
        let op_at = step.find(|c| c == '=' || c == '-').expect("should have operator char");

        let (label, tail) = step.split_at(op_at);
        let op = tail.parse().expect("should be operator");
        
        Step { label, op }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hash_computation() {
        let result = part_one("HASH");
        assert_eq!(result, Some(52));
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1320));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(145));
    }
}
