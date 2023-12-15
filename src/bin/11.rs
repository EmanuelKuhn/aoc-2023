use std::{f32::consts::LN_10, collections::HashSet, mem::swap};

advent_of_code::solution!(11);

type Point = (usize, usize);


fn get_col(grid: &Vec<Vec<char>>, col:usize) -> Vec<char> {
    let mut col_vec = Vec::new();
    
    for row_vec in grid {
        col_vec.push(row_vec[col]);
    }

    col_vec
}

fn iter_cols(grid: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    let mut col_vecs = Vec::new();
    
    for col in 0..grid[0].len() {
        col_vecs.push(get_col(&grid, col));
    }

    col_vecs
}

fn find_in_grid(grid: &Vec<Vec<char>>, target: char) -> Vec<Point> {
    let mut results = Vec::new();

    for (row, row_vec) in grid.iter().enumerate() {
        for (col, value) in row_vec.iter().enumerate() {
            if *value == target {
                results.push((row, col));
            }
        }
    }

    results
}

fn debug_grid(grid: &Vec<Vec<char>>) {
    dbg!(&grid.iter().map(|row| row.iter().map(|&c| c).collect::<String>()).collect::<Vec<_>>());
}

fn pairwise_combinations(galaxies: &Vec<Point>) -> HashSet<(Point, Point)> {

    let mut combinations = HashSet::new();

    for &galaxy_1 in galaxies {
        for &galaxy_2 in galaxies {
            if galaxy_1 < galaxy_2 {
                combinations.insert((galaxy_1, galaxy_2));
            } else if galaxy_2 > galaxy_1 {
                combinations.insert((galaxy_2, galaxy_1));
            }
        }
    }

    combinations
}

fn manhattan_distance(p1: &Point, p2: &Point) -> usize {
    p1.0.abs_diff(p2.0) + p1.1.abs_diff(p2.1)
}

fn process_grid_and_expand(input: &str) -> Vec<Vec<char>>{
    let mut grid: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();

    // debug_grid(&grid);

    let line_insertation_locations: Vec<usize> = (&grid).iter().enumerate().filter(|(_, line)| line.iter().all(|c| *c != '#')).map(|(i, _)| i).collect();

    for (i, loc) in line_insertation_locations.iter().enumerate() {
        let line = vec!['.'; grid[0].len()];
        grid.insert(i + loc, line);
    };

    let column_insert_locations: Vec<usize> = iter_cols(&grid).iter().enumerate().filter(|(_, col)| col.iter().all(|c| *c != '#')).map(|(i, _)| i).collect();

    for row in grid.iter_mut() {
        for (i, loc) in column_insert_locations.iter().enumerate() {
            row.insert(i + *loc, '.');
        };
    }

    return grid
}

pub fn part_one(input: &str) -> Option<usize> {
    let mut grid = process_grid_and_expand(input);
    
    // debug_grid(&grid);

    let galaxies = find_in_grid(&grid, '#');

    let combinations = pairwise_combinations(&galaxies);

    // dbg!(&combinations);

    Some(combinations.iter().map(|(p1, p2)| manhattan_distance(p1, p2)).sum::<usize>() as usize)
}

pub fn part_two(input: &str) -> Option<usize> {
    Some(compute_distance(input, 1000000))
}

fn compute_distance(input: &str, empty_space_amount: usize) -> usize {
    let grid: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();

    let galaxies = find_in_grid(&grid, '#');

    let empty_rows: HashSet<usize> = (&grid).iter().enumerate().filter(|(_, line)| line.iter().all(|c| *c != '#')).map(|(i, _)| i).collect();
    let empty_columns: HashSet<usize> = iter_cols(&grid).iter().enumerate().filter(|(_, col)| col.iter().all(|c| *c != '#')).map(|(i, _)| i).collect();

    let galaxy_pairs = pairwise_combinations(&galaxies);

    galaxy_pairs.iter().map(|(p1, p2)| manhattan_distance_with_empty_space(*p1, *p2, &empty_rows, &empty_columns, empty_space_amount)).sum::<usize>()
}

fn manhattan_distance_with_empty_space(p1: (usize, usize), p2: (usize, usize), empty_rows: &HashSet<usize>, empty_columns: &HashSet<usize>, empty_space_amount: usize) -> usize {
    let (x1, x2) = if p1.0 < p2.0 {(p1.0, p2.0)} else {(p2.0, p1.0)};
    let (y1, y2) = if p1.1 < p2.1 {(p1.1, p2.1)} else {(p2.1, p1.1)};

    let mut x1 = x1;

    let mut distance = 0;

    while x1 < x2 {
        x1 += 1;
        distance += {
            match empty_columns.contains(&(x1 + 1)) {
                true => empty_space_amount,
                false => 1
            }
        }
    }

    let mut y1 = y1;

    while y1 < y2 {
        y1 += 1;
        distance += {
            match empty_rows.contains(&(y1 + 1)) {
                true => empty_space_amount,
                false => 1
            }
        }
    }

    distance
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_expansion() {
        let input = &advent_of_code::template::read_file("examples", DAY);
        let mut grid = process_grid_and_expand(input);

        debug_grid(&grid);

        let expected = "....#........
.........#...
#............
.............
.............
........#....
.#...........
............#
.............
.............
.........#...
#....#.......
";

    let expected: Vec<Vec<char>> = expected.lines().map(|l| l.chars().collect()).collect();

    debug_grid(&expected);

    assert_eq!(grid, expected);
    }

    
    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(374));
    }

    #[test]
    fn test_part_two_10() {
        let result = compute_distance(&advent_of_code::template::read_file("examples", DAY), 10);
        assert_eq!(result, 1030);
    }

    #[test]
    fn test_part_two_100() {
        let result = compute_distance(&advent_of_code::template::read_file("examples", DAY), 100);
        assert_eq!(result, 8410);
    }

    
}
