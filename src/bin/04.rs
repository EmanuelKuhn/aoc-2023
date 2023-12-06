use std::{collections::HashSet, result};

advent_of_code::solution!(4);

pub fn part_one(input: &str) -> Option<u32> {
    let mut result = 0;
    
    for card in input.lines() {

        let overlap = compute_winning_amount(&card);
        if overlap >= 1 {
            result += (2 as u32).pow(overlap as u32 - 1);
        }
    
    }

    Some(result)
}


fn compute_winning_amount(card: &str) -> u32 {
    let card = card.split(':').last().expect("invalid card");

    let splitted: Vec<Vec<&str>> = card.split("|").into_iter()
        .map(|s| s.split_whitespace().collect()).collect();
    
    let my_numbers = splitted.first().expect("my numbers is None");
    let winning_numbers = splitted.last().expect("winning numbers is None");

    let mut my_numbers_set: HashSet<&str> = HashSet::new();
    my_numbers_set.extend(my_numbers);

    let mut winning_numbers_set: HashSet<&str> = HashSet::new();
    winning_numbers_set.extend(winning_numbers);

    let overlap = my_numbers_set.intersection(&winning_numbers_set).count();

    overlap as u32
}

pub fn part_two(input: &str) -> Option<u32> {
    let cards: Vec<&str> = input.lines().collect();

    let scores: Vec<u32> = cards.into_iter().map(|card| compute_winning_amount(card)).collect();

    let mut queue = Vec::new();

    queue.extend(0..scores.len());

    let mut result = 0;

    while let Some(card_index) = queue.pop()  {
        let score = scores[card_index];

        result += 1;

        for new_index in (card_index+1)..=(card_index+score as usize) {
            if new_index < scores.len() {
                queue.push(new_index);
            }
        }
    }

    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(13));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(30));
    }
}
