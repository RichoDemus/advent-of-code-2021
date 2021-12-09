use itertools::Itertools;
use std::cmp::Ordering;
use std::collections::{HashMap, HashSet};

#[aoc_generator(day9)]
fn parse_input(input: &str) -> Vec<Vec<i64>> {
    input
        .lines()
        .map(|line| {
            line.chars()
                .map(|char| char.to_digit(10).unwrap())
                .map(i64::from)
                .collect()
        })
        .collect()
}

#[aoc(day9, part1)]
fn part1(input: &[Vec<i64>]) -> i64 {
    let mut risk_level = 0;
    for (y, row) in input.iter().enumerate() {
        for (x, point) in row.iter().enumerate() {
            let north_neihbour = y
                .checked_sub(1)
                .and_then(|y| input.get(y))
                .and_then(|row| row.get(x));
            let south_neihbour = input.get(y + 1).and_then(|row| row.get(x));
            let east_neihbour = input.get(y).and_then(|row| row.get(x + 1));
            let west_neihbour = input
                .get(y)
                .and_then(|row| x.checked_sub(1).and_then(|x| row.get(x)));
            // println!("{} at ({},{}). neighbours: N/S/E/W: {:?} {:?} {:?} {:?} ", point, x,y, north_neihbour,south_neihbour,east_neihbour,west_neihbour);
            if let Some(height) = north_neihbour {
                if point >= height {
                    continue;
                }
            }
            if let Some(height) = south_neihbour {
                if point >= height {
                    continue;
                }
            }
            if let Some(height) = east_neihbour {
                if point >= height {
                    continue;
                }
            }
            if let Some(height) = west_neihbour {
                if point >= height {
                    continue;
                }
            }
            risk_level += *point + 1;
        }
    }
    risk_level
}

#[aoc(day9, part2)]
fn part2(input: &[Vec<i64>]) -> usize {
    // a "lowest number", expand around neighbours while all new numbers are bigger
    let mut map = HashMap::new();
    for (y, row) in input.iter().enumerate() {
        for (x, point) in row.iter().enumerate() {
            map.insert((x as i64, y as i64), *point);
        }
    }

    let mut basin_sizes = vec![];
    let mut coordinates_already_in_another_basin = HashSet::new();
    for start_level in 0..=9 {
        let coordinates = map
            .iter()
            .filter(|(_coordinates, level)| **level == start_level)
            .filter(|(_coordinates, level)| **level != 9)
            .map(|(coordinates, _)| *coordinates)
            .collect::<Vec<_>>();

        for (x_start, y_start) in coordinates {
            if coordinates_already_in_another_basin.contains(&(x_start, y_start)) {
                continue;
            }
            let mut basin = HashSet::new();
            basin.insert((x_start, y_start));
            // expand basin
            // currently look for neighbours that are look-level high
            for look_level in start_level..=8 {
                // println!("\tLook level is now {}", look_level);
                let mut new_coordinates_to_add_to_basin = HashSet::new();
                for (current_x, current_y) in &basin {
                    for (neighbour_x, neighbour_y, _direction) in
                        calc_neighbours(*current_x, *current_y)
                    {
                        // println!("\tAs {},{} looking to see if {:?} neighbour {:?} is in same basin", current_x, current_y, direction, map.get(&(neighbour_x, neighbour_y)));
                        if coordinates_already_in_another_basin
                            .contains(&(neighbour_x, neighbour_y))
                        {
                            panic!("We've gotten into another basin! :O")
                        }
                        if basin.contains(&(neighbour_x, neighbour_y)) {
                            // this point is already in this basin
                            continue;
                        }
                        let neighbour_height = match map.get(&(neighbour_x, neighbour_y)) {
                            None => continue,
                            Some(neighbour_height) => *neighbour_height,
                        };
                        match neighbour_height.cmp(&look_level) {
                            Ordering::Less => panic!(
                                "Found a new basin I think: {},{}, {}",
                                neighbour_x, neighbour_y, neighbour_height
                            ),
                            Ordering::Equal => {
                                // same height as what we're looking at
                                // println!("\t\tAdded {},{} to current basin", neighbour_x, neighbour_y);
                                new_coordinates_to_add_to_basin.insert((neighbour_x, neighbour_y));
                            }
                            Ordering::Greater => (), // this neighbour is higher up, ignore it, maybe it will be covered at the next look level
                        }
                    }
                }
                for coord in new_coordinates_to_add_to_basin {
                    basin.insert(coord);
                }
            }
            // println!("Found a size {} basin? {:?}", basin.len(), basin);
            basin_sizes.push(basin.len());
            for coord in basin {
                coordinates_already_in_another_basin.insert(coord);
            }
        }
    }

    basin_sizes.into_iter().sorted().rev().take(3).product()
}

#[derive(Debug)]
enum Direction {
    N,
    E,
    S,
    W,
}

fn calc_neighbours(x: i64, y: i64) -> Vec<(i64, i64, Direction)> {
    vec![
        (x + 1, y, Direction::E),
        (x - 1, y, Direction::W),
        (x, y + 1, Direction::N),
        (x, y - 1, Direction::S),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn verify_part1() {
        let input = include_str!("../input/2021/day9.txt");
        let input = parse_input(input);
        assert_eq!(part1(input.as_slice()), 494);
    }

    #[test]
    fn verify_part2() {
        let input = include_str!("../input/2021/day9.txt");
        assert_eq!(part2(parse_input(input).as_slice()), 1048128);
    }

    #[test]
    fn part1_provided_example() {
        let result = part1(&parse_input(
            r#"2199943210
3987894921
9856789892
8767896789
9899965678"#,
        ));

        assert_eq!(result, 15)
    }

    #[test]
    fn part2_provided_example() {
        let result = part2(&parse_input(
            r#"2199943210
3987894921
9856789892
8767896789
9899965678"#,
        ));

        assert_eq!(result, 1134)
    }
}
