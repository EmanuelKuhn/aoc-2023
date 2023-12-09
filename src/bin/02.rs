advent_of_code::solution!(2);

use std::{str::FromStr, fmt::{self}, collections::HashMap};


// type Map<T1, T2> = BTreeMap<Vec<T1>, T2>

fn verify_game(game: &Game) -> bool {

    let constraints = [("red", 12), ("green", 13), ("blue", 14)];

    for set in &game.records {
        for (item, cnt) in set {
            for (constraint_item, max) in constraints {
                if item == constraint_item {
                    if cnt > &max {
                        return false
                    }
                }
            }
        }
    }

    return true
}

fn power_of_game(game: &Game) -> u32 {
    // let minimums: HashMap<&str, u32> = vec![("red", 0), ("green", 0), ("blue", 0)].into_iter().collect();
    let mut minimums: HashMap<&str, u32> = HashMap::new();

    for set in &game.records {
        for (item, cnt) in set {
            let entry = minimums.entry(item).or_insert(0);
            *entry = std::cmp::max(*entry, *cnt);
        }
    }

    return minimums.into_values().product();
}

#[derive(Debug, PartialEq)]
struct Game {
    id: u32,
    records: Vec<Vec<(String, u32)>>
}


#[derive(Debug)]
pub enum MyCustomError {
  ParseError,
  ParseIntError,
}

impl From<std::num::ParseIntError> for MyCustomError{
    fn from(_value: std::num::ParseIntError) -> Self {
        MyCustomError::ParseIntError
    }
}

impl std::error::Error for MyCustomError {}

impl fmt::Display for MyCustomError {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match self {
      MyCustomError::ParseError => write!(f, "Parse Error"),
      MyCustomError::ParseIntError => write!(f, "Parse Int Error")
    }
  }
}

impl FromStr for Game {
    // type Err = std::num::ParseIntError;
    type Err = MyCustomError;

    // Parses a color hex code of the form '#rRgGbB..' into an
    // instance of 'RGB'
    fn from_str(line: &str) -> Result<Self, Self::Err> {

        let colon_location = line.find(":").ok_or_else(|| MyCustomError::ParseError)?;

        let game_id_string = &line[5..colon_location];

        let game_id = game_id_string.parse::<u32>()?;

        let sets_string = &line[colon_location+1..];

        let mut records = vec![];

        for set_description in sets_string.split(";") {
            let set = parse_set(set_description)?;

            records.push(set);
        }

        Ok(Game { id: game_id, records: records })
    }
}

impl fmt::Display for Game {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Game {}: {:?}", self.id, self.records)
    }
  }

fn parse_set(set_description: &str) -> Result<Vec<(String, u32)>, MyCustomError> {
    // dbg!(set_description);

    let mut items = vec![];

    for item in set_description.split(",") {
        let item = item.trim();

        let count_and_item: Vec<&str> = item.split(" ").collect();

        // dbg!(&count_and_item);

        let count = count_and_item[0].parse::<u32>()?;
        let item = count_and_item[1];

        // dbg!(count);
        // dbg!(item);

        items.push((item.to_owned(), count));
    }
    
    return Ok(items);
}


pub fn part_one(input: &str) -> Option<u32> {
    let mut verified_ids: Vec<u32> = vec![];

    for line in input.lines() {
        match line.parse::<Game>() {
            Ok(game) => {
                if verify_game(&game) {
                    verified_ids.push(game.id)
                }
            },
            Err(e) => println!("Failed to parse: {}", e)
        }
    }

    Some(verified_ids.iter().sum())
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut sum_of_powers = 0;

    for line in input.lines() {
        match line.parse::<Game>() {
            Ok(game) => {
                sum_of_powers += power_of_game(&game);
            },
            Err(e) => println!("Failed to parse: {}", e)
        }
    }

    Some(sum_of_powers)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(8));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2286));
    }

    #[test]
    fn test_parse_game_id() {
        let input = "Game 100: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green";

        let game = Game::from_str(input);

        assert_eq!(game.unwrap().id, 100)
    }

    #[test]
    fn test_parse_records() {
        let input = "Game 100: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green";

        let game = Game::from_str(input);

        // assert_eq!(game.unwrap().id, 100)

        let game = game.unwrap();
        
        let expected = vec![make_expected_map(vec![("blue", 3), ("red", 4)]), 
        make_expected_map(vec![("red", 1), ("green", 2), ("blue", 6)]), make_expected_map(vec![("green", 2)])];

        assert_eq!(game.records, expected);
    }

    fn make_expected_map(mapping: Vec<(&str, u32)>) -> Vec<(String, u32)> {
        return mapping.iter().map(|(item, cnt)| (item.to_string().to_owned(), cnt.to_owned())).collect();
    }

    #[test]
    fn test_parse_set_description() {
        let input = " 1 red, 2 green, 6 blue";

        let set = parse_set(input);

        let set = set.unwrap();

        let expected: Vec<(String, u32)> = make_expected_map(vec![("red", 1), ("green", 2), ("blue", 6)]);
        
        

        assert_eq!(set, expected);
    }
}
