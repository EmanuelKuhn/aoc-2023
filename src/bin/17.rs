use std::{collections::{HashMap, HashSet, BinaryHeap}, panic::Location};

advent_of_code::solution!(17);

#[derive(Eq, Hash, PartialEq, Debug, Clone, Copy, PartialOrd, Ord)]
struct Key {
    location: Point,
    steps_in_direction: u8,
    direction: Direction, 
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
struct State {
    cost: u64,
    key: Key
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.cost.cmp(&self.cost).then_with(|| self.key.cmp(&other.key))
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Eq, Hash, PartialEq, Debug, Clone, Copy, PartialOrd, Ord)]
struct Point {x: usize, y: usize}

type Cell = u8;

struct CityGrid {
    cells: HashMap<Point, Cell>,
    // finish_location: Point,
}


impl CityGrid {
    fn new() -> Self { Self { cells: HashMap::new()} }

    fn find_path(&mut self, start: Point, finish: Point) -> u64 {

        let mut discovered: BinaryHeap<State> = BinaryHeap::new();

        let mut seen: HashSet<Key> = HashSet::new();
        
        discovered.push(State { cost: 0, key: Key { location: start, direction: Direction::Right, steps_in_direction: 0 } });

        while let Some(state) = discovered.pop() {

            if state.key.location == finish {
                return state.cost
            }

            if seen.contains(&state.key) {
                continue
            }

            seen.insert(state.key);

            let mut candidate_directions = vec![state.key.direction.turn_left(), state.key.direction.turn_right()];

            if state.key.steps_in_direction < 3 {
                candidate_directions.push(state.key.direction);
            }

            for new_direction in candidate_directions {
                if let Some(new_location) = self.get_neighbor(&state.key.location, &new_direction) {
                    let new_cost = state.cost + self.cells[&new_location] as u64;

                    let new_steps_in_direction = match state.key.direction == new_direction {
                        true => state.key.steps_in_direction + 1,
                        false => 1, 
                    };

                    discovered.push(
                    State { 
                            cost: new_cost, 
                            key: Key { location: new_location, direction: new_direction, steps_in_direction: new_steps_in_direction } 
                        }
                    )
                }
            }
        }

        panic!("Didn't reach finish")


    }

    fn find_path_2(&mut self, start: Point, finish: Point) -> u64 {

        let mut discovered: BinaryHeap<State> = BinaryHeap::new();

        let mut seen: HashSet<Key> = HashSet::new();


        discovered.push(State { cost: 0, key: Key { location: start, direction: Direction::Left, steps_in_direction: 0 } });
        // discovered.push(State { cost: 0, key: Key { location: start, direction: Direction::Down, steps_in_direction: 0 } });

        let mut dists: HashMap<Point, u64> = HashMap::new();

        while let Some(state) = discovered.pop() {

            if state.key.steps_in_direction >= 4 {
                dists.entry(state.key.location).and_modify(|e| {*e = *e.min(&mut state.cost.clone())}).or_insert(state.cost);
            }

            // if state.key.location == finish && state.key.steps_in_direction >= 4 {
            //     return state.cost
            // }

            if seen.contains(&state.key) {
                continue
            }

            seen.insert(state.key);

            let mut candidate_directions = Vec::new();

            let current_direction = state.key.direction;

            match state.key.steps_in_direction {
                0 => {
                    // Only 0 when at the start
                    candidate_directions.append(&mut vec![Direction::Left, Direction::Right, Direction::Up, Direction::Down]);
                }
                1..=3 => candidate_directions.push(current_direction),
                4..=9 => candidate_directions.append(&mut vec![current_direction, current_direction.turn_left(), current_direction.turn_right()]),
                10 => candidate_directions.append(&mut vec![current_direction.turn_left(), current_direction.turn_right()]),
                11.. => panic!("should not be possible to continue straight for more than 10")
            }

            for new_direction in &candidate_directions {
                if let Some(new_location) = self.get_neighbor(&state.key.location, new_direction) {
                    let new_cost = state.cost + self.cells[&new_location] as u64;

                    let new_steps_in_direction = match state.key.direction == *new_direction {
                        true => state.key.steps_in_direction + 1,
                        false => 1,
                    };

                    let new_state = State { 
                            cost: new_cost, 
                            key: Key { location: new_location, direction: *new_direction, steps_in_direction: new_steps_in_direction } 
                    };

                    discovered.push(new_state);
                    
                }
            }
        }

        for y in 0..=finish.y {
            for x in 0..=finish.x {
                if let Some(&hl) = dists.get(&Point { x, y }) {
                    print!("{:03} ", hl);
                } else {
                    print!("    ");
                }
            }
            println!();
        }

        // panic!("Didn't reach finish")

        *dists.get(&finish).unwrap()


    }

    fn get_neighbor(&self, point: &Point, direction: &Direction) -> Option<Point> {
        let proposal = match direction {
            Direction::Down => Point {x: point.x, y: point.y + 1},
            Direction::Right => Point {x: point.x + 1, y: point.y},
            Direction::Up => if point.y > 0 {Point {x: point.x, y: point.y - 1}} else {return None},
            Direction::Left => if point.x > 0 {Point {x: point.x - 1, y: point.y}} else {return None},
        };

        match self.cells.contains_key(&proposal) {
            true => Some(proposal),
            false => None,
        }
    }
}

#[derive(Hash, PartialEq, Eq, Debug, Clone, Copy, PartialOrd, Ord)]
enum Direction {
    Up, Right, Down, Left
}

impl Direction {
    fn turn_left(&self) -> Self {
        match self {
            Direction::Up => Direction::Left,
            Direction::Right => Direction::Up,
            Direction::Down => Direction::Right,
            Direction::Left => Direction::Down,
        }
    }

    fn turn_right(&self) -> Self {
        match self {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
        }
    }
}

fn parse_grid(input: &str) -> (CityGrid, Point) {
    let width = input.lines().nth(0).unwrap().chars().count();
    let height = input.lines().count();

    let finish = Point { x: width - 1, y: height - 1 };

    let mut grid: CityGrid = CityGrid::new();

    for (row, line) in input.lines().enumerate() {
        for (col, c) in line.chars().enumerate() {
            let location = Point {x: col, y: row};
            grid.cells.insert(location.clone(), c.to_string().as_str().parse().expect("should be <='9'"));
        }
    }

    (grid, finish)
}

pub fn part_one(input: &str) -> Option<u64> {
    let (grid, finish) = parse_grid(&input);

    let mut grid = grid;

    return Some(grid.find_path(Point { x: 0, y: 0 }, finish))
}

pub fn part_two(input: &str) -> Option<u64> {
    let (grid, finish) = parse_grid(&input);

    let mut grid = grid;

    return Some(grid.find_path_2(Point { x: 0, y: 0 }, finish))

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(102));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(94));
    }

    #[test]
    fn test_part_two_two() {
        let result = part_two(&advent_of_code::template::read_file_part("examples", DAY, 2));
        assert_eq!(result, Some(71));
    }
}
