use std::collections::{HashMap, HashSet};
advent_of_code::solution!(16);


#[derive(Eq, Hash, PartialEq, Debug, Clone, Copy)]
struct Point {x: usize, y: usize}

struct LightGrid {
    cells: HashMap<Point, Cell>,
}
struct Cell {
    cell_type: char,
    beams: HashSet<Direction>,
    activated: bool,
}

#[derive(Hash, PartialEq, Eq, Debug, Clone, Copy)]
enum Direction {
    Up, Right, Down, Left
}

impl LightGrid {
    fn new(capacity: usize) -> Self { Self { cells: HashMap::with_capacity(capacity) } }

    fn activate(&mut self, location: Point, beam_direction: Direction) {
        let cell = self.cells.get_mut(&location).unwrap();

        cell.activated = true;

        // println!("activated {:?} for {:?}", &location, beam_direction);

        if cell.beams.contains(&beam_direction) {
            // Stop recursion if already handled this incoming direction

            // println!("stopping recursion {:?} for {:?}", &location, beam_direction);
            return;
        }

        cell.beams.insert(beam_direction.clone());

        let new_directions = match cell.cell_type {
            '.' => vec![beam_direction],
            '/' => match beam_direction {
                Direction::Up => vec![Direction::Right],
                Direction::Right => vec![Direction::Up],
                Direction::Down => vec![Direction::Left],
                Direction::Left => vec![Direction::Down]
            },
            '\\' => match beam_direction {
                Direction::Up => vec![Direction::Left],
                Direction::Right => vec![Direction::Down],
                Direction::Down => vec![Direction::Right],
                Direction::Left => vec![Direction::Up]
            },
            '|' => match beam_direction {
                Direction::Up | Direction::Down => vec![beam_direction],
                Direction::Left | Direction::Right => vec![Direction::Up, Direction::Down]
            },
            '-' => match beam_direction {
                Direction::Left | Direction::Right => vec![beam_direction],
                Direction::Up | Direction::Down => vec![Direction::Left, Direction::Right],
            },
            _ => panic!("unexpected cell type")
        };

        for new_direction in new_directions {
            if let Some(new_location) = self.get_neighbor(&location, &new_direction) {
                self.activate(new_location, new_direction);
            }
        }
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

pub fn part_one(input: &str) -> Option<u64> {


    Some(count_activated(&input, Point { x: 0, y: 0 }, Direction::Right))
}

fn count_activated(input: &str, start_point: Point, beam_direction: Direction) -> u64 {
    let mut grid = parse_grid(input);

    grid.activate(start_point, beam_direction);

    grid.cells.values().filter(|c| c.activated).count() as u64
}

fn parse_grid(input: &str) -> LightGrid {
    let mut grid: LightGrid = LightGrid::new(input.len());

    for (row, line) in input.lines().enumerate() {
        for (col, c) in line.chars().enumerate() {
            let location = Point {x: col, y: row};
            grid.cells.insert(location.clone(), Cell { cell_type: c, activated: false, beams: HashSet::new() });
        }
    }
    grid
}

pub fn part_two(input: &str) -> Option<u64> {
    let width = input.lines().nth(0).unwrap().chars().count();
    let height = input.lines().count();

    let mut activated = Vec::new();

    for x_start in 0..width {
        activated.push(count_activated(&input, Point { x: x_start, y: 0 }, Direction::Down));

        activated.push(count_activated(&input, Point { x: x_start, y: height - 1 }, Direction::Up));
    }

    for y_start in 0..height {
        activated.push(count_activated(&input, Point { x: 0, y: y_start }, Direction::Right));

        activated.push(count_activated(&input, Point { x: width - 1, y: y_start }, Direction::Left));
    }

    activated.push(count_activated(&input, Point { x: width - 1, y: height - 1 }, Direction::Up));
    activated.push(count_activated(&input, Point { x: width - 1, y: height - 1 }, Direction::Left));

    activated.push(count_activated(&input, Point { x: width - 1, y: 0 }, Direction::Down));
    activated.push(count_activated(&input, Point { x: width - 1, y: 0 }, Direction::Left));

    activated.push(count_activated(&input, Point { x: 0, y: height - 1 }, Direction::Up));
    activated.push(count_activated(&input, Point { x: 0, y: height - 1 }, Direction::Right));

    activated.push(count_activated(&input, Point { x: 0, y: 0 }, Direction::Down));
    activated.push(count_activated(&input, Point { x: 0, y: 0 }, Direction::Right));

    activated.into_iter().max()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(46));
    }

    #[test]
    fn test_part_two() {
        let result: Option<u64> = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(51));
    }
}
