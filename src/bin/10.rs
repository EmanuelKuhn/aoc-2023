use std::{
    char,
    cmp::Reverse,
    collections::{BinaryHeap, HashMap, HashSet},
    fmt::{self, Debug, Display},
    str::FromStr,
};

use Tile::*;

advent_of_code::solution!(10);

struct Grid<T: PartialEq + Copy> {
    arr: Vec<T>,
    pub width: usize,
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash, Debug)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl Display for Direction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Direction::Up => write!(f, "Up"),
            Direction::Right => write!(f, "Right"),
            Direction::Down => write!(f, "Down"),
            Direction::Left => write!(f, "Left"),
        }
    }
}

type Point = (usize, usize);

impl Direction {
    fn opposite(&self) -> Self {
        match &self {
            Self::Up => Self::Down,
            Self::Right => Self::Left,
            Self::Down => Self::Up,
            Self::Left => Self::Right,
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
enum Tile {
    NorthSouth = b'|',
    EastWest = b'-',
    NorthEast = b'L',
    NortWest = b'J',
    SouthWest = b'7',
    SouthEast = b'F',
    Ground = b'.',
    Start = b'S',
}

impl Display for Tile {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let c: char = (*self).into();

        write!(f, "{}", c)
    }
}

impl Into<char> for Tile {
    fn into(self) -> char {
        self as u8 as char
    }
}

impl TryFrom<char> for Tile {
    type Error = ();

    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            c if c == NorthSouth.into() => Ok(NorthSouth),
            c if c == EastWest.into() => Ok(EastWest),
            c if c == NorthEast.into() => Ok(NorthEast),
            c if c == NortWest.into() => Ok(NortWest),
            c if c == SouthWest.into() => Ok(SouthWest),
            c if c == SouthEast.into() => Ok(SouthEast),
            c if c == Ground.into() => Ok(Ground),
            c if c == Start.into() => Ok(Start),
            _ => Err(()),
        }
    }
}

impl Tile {
    fn is_connected_to(&self, direction: &Direction) -> bool {
        match (&self, direction) {
            (NorthSouth, Direction::Up | Direction::Down) => true,
            (EastWest, Direction::Left | Direction::Right) => true,
            (NorthEast, Direction::Up | Direction::Right) => true,
            (NortWest, Direction::Up | Direction::Left) => true,
            (SouthWest, Direction::Down | Direction::Left) => true,
            (SouthEast, Direction::Down | Direction::Right) => true,
            (Ground, _) => false,
            (Start, _) => true,
            _ => false,
        }
    }
}

impl<T: PartialEq + Copy> Grid<T> {
    // type Err = String;

    fn get(&self, p: Point) -> T {
        let (x, y) = p;

        assert!(x < self.width);
        assert!(y < self.height());

        let index = self.width * y + x;

        assert!(index < self.arr.len());

        self.arr[index]
    }

    fn get_mut(&mut self, p: Point) -> &mut T {
        let (x, y) = p;

        let index = self.width * y + x;

        assert!(index < self.arr.len());

        self.arr.get_mut(index).unwrap()
    }

    fn replace_sub_grid(&mut self, at: Point, data: Vec<T>, dst_width: usize) {
        assert!(data.len() % dst_width == 0);

        let (x, y) = at;

        let n_lines = data.len() / dst_width;

        assert!(x + dst_width <= self.width);
        assert!(y + n_lines <= self.height());

        for line in 0..n_lines {
            let dst_range =
                (self.width * (y + line) + x)..(self.width * (y + line) + x + dst_width);

            self.arr[dst_range].copy_from_slice(&data[line * dst_width..((line + 1) * dst_width)])
        }
    }

    fn find_value(&self, value: T) -> Option<Point> {
        let index: usize = self.arr.iter().position(|c| *c == value)?;

        let y = index / self.width;
        let x = index - (y * self.width);

        Some((x, y).into())
    }

    fn index_to_xy(&self, index: usize) -> Point {
        assert!(index < self.arr.len());

        let y = index / self.width;
        let x = index - (y * self.width);

        (x, y)
    }

    fn neighbor_locations(&self, p: Point) -> Vec<(Point, Direction)> {
        let (x, y) = p;

        assert!(x < self.width);
        assert!(y < self.height());

        let mut neighbors: Vec<(Point, Direction)> = Vec::new();

        if x > 0 {
            neighbors.push(((x - 1, y), Direction::Left));
        }

        if x + 1 < self.width {
            neighbors.push(((x + 1, y), Direction::Right));
        }

        if y > 0 {
            neighbors.push(((x, y - 1), Direction::Up));
        }

        let height = self.height();

        if y + 1 < height {
            neighbors.push(((x, y + 1), Direction::Down));
        }

        neighbors
    }

    fn height(&self) -> usize {
        self.arr.len() / self.width
    }

    fn touches_edge(&self, p: Point) -> bool {
        self.neighbor_locations(p).len() < 4
    }
}

impl Grid<Tile> {
    fn connected_neighbors(&self, p: Point) -> Vec<Point> {
        let current = self.get(p);

        let mut result = Vec::new();

        for (p_other, direction) in self.neighbor_locations(p) {
            let other = self.get(p_other);

            let is_connected =
                current.is_connected_to(&direction) && other.is_connected_to(&direction.opposite());

            if is_connected {
                result.push((p_other));
            }
        }

        result
    }
}

impl<T: Copy + PartialEq> Grid<T> {
    fn filled(width: usize, height: usize, value: T) -> Grid<T> {
        let arr = vec![value; width * height];

        Grid {
            arr: arr,
            width: width,
        }
    }
}

impl FromStr for Grid<Tile> {
    type Err = String;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let lines: Vec<&str> = input.lines().collect();

        assert!(lines.len() >= 1);

        let width = lines[0].len();

        let arr: Vec<Tile> = lines
            .iter()
            .flat_map(|l| {
                l.chars()
                    .map(|c| c.try_into().expect("should parse all tiles"))
            })
            .collect();

        Ok(Grid {
            arr: arr,
            width: width,
        })
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let grid: Grid<Tile> = input.parse().expect("should parse grid input");

    let start_pos = grid
        .find_value('S'.try_into().unwrap())
        .expect("should find start S");

    // map from seen node to distance
    let mut seen: HashMap<(Point), u32> = HashMap::new();

    seen.insert(start_pos, 0);

    let mut queue = BinaryHeap::new();

    queue.push((Reverse(0), start_pos));

    while queue.len() > 0 {
        let (distance, p) = queue.pop().expect("should have at least one item");

        for (nx, ny) in grid.connected_neighbors(p) {
            if seen.contains_key(&(nx, ny)) {
                assert!(*seen.get(&(nx, ny)).unwrap() <= distance.0 + 1);
            } else {
                seen.insert((nx, ny), distance.0 + 1);
                queue.push((Reverse(distance.0 + 1), (nx, ny)));
            }
        }
    }

    Some(*seen.values().max().unwrap())
}

#[derive(PartialEq, Clone, Copy)]
enum State<T: Display> {
    Unkown,
    Inside,
    Outside,
    Loop(T),
}

impl<T: Display> fmt::Display for State<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Unkown => write!(f, " "),
            Self::Inside => write!(f, "I"),
            Self::Outside => write!(f, " "),
            Self::Loop(v) => write!(f, "{}", v),
        }
    }
}

impl<T: Display + PartialEq + Clone + Copy> fmt::Debug for Grid<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        Ok(for (i, val) in self.arr.iter().enumerate() {
            if i % self.width == 0 {
                writeln!(f);
            }

            write!(f, "{}", val);
        })
    }
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut grid: Grid<Tile> = input.parse().expect("should parse grid input");

    let loop_dists = compute_loop(&mut grid);

    let loop_locations: HashSet<&(Point)> = loop_dists.keys().collect();

    let mut classifications: Grid<State<Tile>> =
        Grid::filled(grid.width, grid.height(), State::Unkown);

    for p in &loop_locations {
        *classifications.get_mut(**p) = State::Loop(grid.get(**p));
    }

    dbg!(&classifications);

    let mut classifications = enlargen_classifications(classifications);

    dbg!(&classifications);

    while let Some(seed_point) = classifications.find_value(State::Unkown) {
        let mut seen = HashSet::new();
        seen.insert(seed_point);

        let mut queue = Vec::new();
        queue.push(seed_point);

        let mut enclosed = true;

        while let Some(p) = queue.pop() {
            match classifications.get(p) {
                Loop(_) => (),
                _ => {
                    let neighbors = classifications.neighbor_locations(p);

                    for (n_point, _) in neighbors {
                        if !seen.contains(&n_point)
                            && !matches!(classifications.get(n_point), Loop(_))
                        {
                            seen.insert(n_point);

                            queue.push(n_point);

                            if classifications.touches_edge(p) {
                                enclosed = false;
                            }
                        }
                    }
                }
            }
        }

        for point in seen {
            *classifications.get_mut(point) = if enclosed {
                State::Inside
            } else {
                State::Outside
            };
        }
    }

    dbg!(&classifications);

    Some(
        classifications
            .arr
            .iter()
            .enumerate()
            .filter_map(|(i, el)| {
                let (x, y) = classifications.index_to_xy(i);

                if x > 0 && (x - 1) % 3 == 0 && y > 0 && (y - 1) % 3 == 0 {
                    Some(el)
                } else {
                    None
                }
            })
            .filter(|s| **s == State::Inside)
            .count() as u32,
    )
}

type T = Tile;
fn enlargen_classifications(classifications: Grid<State<Tile>>) -> Grid<State<T>> {
    let new_width = classifications.width * 3;
    let new_height = classifications.height() * 3;

    let mut new_grid: Grid<State<T>> = Grid::filled(new_width, new_height, State::Unkown);

    for (index, tile) in classifications.arr.iter().enumerate() {
        let (x, y) = classifications.index_to_xy(index);

        let new_x = x * 3;
        let new_y = y * 3;

        let new_index = new_y * new_width + new_x;

        let new_values: Vec<State<T>> = map_tile_to_3x3(*tile);

        assert!(new_values.len() == 9);

        new_grid.replace_sub_grid((new_x, new_y), new_values, 3);
    }

    new_grid
}

use State::*;

fn map_tile_to_3x3(tile: State<Tile>) -> Vec<State<T>> {
    match tile {
        State::Unkown => vec![tile; 9],
        State::Inside => vec![tile; 9],
        State::Outside => vec![tile; 9],
        State::Loop(loop_tile) => match loop_tile {
            NorthSouth => vec![
                Unkown,
                Loop(NorthSouth),
                Unkown,
                Unkown,
                Loop(NorthSouth),
                Unkown,
                Unkown,
                Loop(NorthSouth),
                Unkown,
            ],
            EastWest => vec![
                Unkown,
                Unkown,
                Unkown,
                Loop(EastWest),
                Loop(EastWest),
                Loop(EastWest),
                Unkown,
                Unkown,
                Unkown,
            ],
            NorthEast => vec![
                Unkown,
                Loop(NorthSouth),
                Unkown,
                Unkown,
                Loop(NorthEast),
                Loop(EastWest),
                Unkown,
                Unkown,
                Unkown,
            ],
            NortWest => vec![
                Unkown,
                Loop(NorthSouth),
                Unkown,
                Loop(EastWest),
                Loop(NortWest),
                Unkown,
                Unkown,
                Unkown,
                Unkown,
            ],
            SouthWest => vec![
                Unkown,
                Unkown,
                Unkown,
                Loop(EastWest),
                Loop(SouthWest),
                Unkown,
                Unkown,
                Loop(NorthSouth),
                Unkown,
            ],
            SouthEast => vec![
                Unkown,
                Unkown,
                Unkown,
                Unkown,
                Loop(SouthEast),
                Loop(EastWest),
                Unkown,
                Loop(NorthSouth),
                Unkown,
            ],
            Ground => vec![Unkown; 9],
            Start => panic!("Should not be in the grid"),
        },
    }
}

fn compute_loop(grid: &mut Grid<Tile>) -> HashMap<(Point), u32> {
    let start_pos = grid
        .find_value('S'.try_into().unwrap())
        .expect("should find start S");

    // map from seen node to distance
    let mut seen: HashMap<Point, (u32, Option<Direction>)> = HashMap::new();

    // let mut start_direction: HashMap<Point, Direction> = HashMap::new();

    seen.insert(start_pos, (0, None));

    let mut queue = BinaryHeap::new();

    for (neighbor_pos, direction) in grid
        .neighbor_locations(start_pos)
        .into_iter()
        .filter(|(p, _)| grid.connected_neighbors(start_pos).contains(p))
    {
        queue.push((Reverse(1), neighbor_pos, direction));
    }

    let mut connected_directions = HashSet::new();

    while queue.len() > 0 {
        let (distance, point, start_direction) =
            queue.pop().expect("should have at least one item");

        for np in grid.connected_neighbors(point) {
            if seen.contains_key(&np) {
                let (neighbor_distance, neighbor_start_direction) = *seen.get(&np).unwrap();

                assert!(neighbor_distance <= distance.0 + 1);

                if let Some(neighbor_start_direction) = neighbor_start_direction {
                    if start_direction != neighbor_start_direction {
                        connected_directions.insert(start_direction);
                        connected_directions.insert(neighbor_start_direction);

                        assert!(connected_directions.len() == 2);
                    }
                }

                // assert!(Some(start_direction) == neighbor_start_direction);
            } else {
                seen.insert(np, (distance.0 + 1, Some(start_direction)));
                queue.push((Reverse(distance.0 + 1), np, start_direction));
            }
        }
    }

    // let connected_directions: Vec<Direction> = connected_directions.into_iter().collect();

    // let connected_directions

    let start_tile = match &connected_directions {
        d if d.contains(&Direction::Up) && d.contains(&Direction::Down) => Tile::NorthSouth,
        d if d.contains(&Direction::Left) && d.contains(&Direction::Right) => Tile::EastWest,
        d if d.contains(&Direction::Up) && d.contains(&Direction::Right) => Tile::NorthEast,
        d if d.contains(&Direction::Up) && d.contains(&Direction::Left) => Tile::NortWest,
        d if d.contains(&Direction::Down) && d.contains(&Direction::Right) => Tile::SouthEast,
        d if d.contains(&Direction::Down) && d.contains(&Direction::Left) => Tile::SouthWest,
        ds => panic!(
            "Unhandled case: {:?}",
            ds.iter().collect::<Vec<&Direction>>()
        ),
    };

    *grid.get_mut(start_pos) = start_tile;

    HashMap::from_iter(seen.iter().filter_map(|(key, (dist, direction))| {
        if match direction {
            Some(direction) => connected_directions.contains(direction),
            None => true,
        } {
            Some((*key, *dist))
        } else {
            None
        }
    }))
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
    fn test_grid_find() {
        let input = &advent_of_code::template::read_file("examples", DAY);

        let grid = input.parse::<Grid<Tile>>().unwrap();

        let start_pos = grid.find_value('S'.try_into().unwrap());

        assert_eq!(start_pos, Some((0, 2)));
    }

    #[test]
    fn test_find_start_neighbors() {
        let input = &advent_of_code::template::read_file("examples", DAY);

        let grid = input.parse::<Grid<Tile>>().unwrap();

        let sp = grid.find_value('S'.try_into().unwrap()).unwrap();

        let result = grid.connected_neighbors((sp.0 + 1, sp.1));

        assert_eq!(result, vec![(0, 2), (1, 1)]);
    }

    #[test]
    fn test_part_two_one() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(8));
    }
}
