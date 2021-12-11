use std::collections::{HashMap, HashSet};

use crate::grid::{get_eight_neighbours, Grid};

#[aoc_generator(day11)]
fn parse_input(input: &str) -> Grid<u8> {
    Grid::from_non_delim_block(input)
}

#[aoc(day11, part1)]
fn part1(grid: &Grid<u8>) -> usize {
    let mut grid = grid.clone();

    let mut flashes = 0;
    for _step in 1..=100 {
        // first increase all values by 1
        for ((_x, _y), energy_level) in &mut grid.grid {
            *energy_level += 1;
        }
        //flash
        let mut did_flash_this_step = HashSet::new();
        'inner: loop {
            let mut next_to_a_flash = HashMap::new();
            for ((x, y), energy_level) in &mut grid.grid {
                if did_flash_this_step.contains(&(*x, *y)) {
                    continue;
                }
                if *energy_level > 9 {
                    did_flash_this_step.insert((*x, *y));
                    flashes += 1;
                    for (x_neighbour, y_neighbour) in get_eight_neighbours(*x, *y) {
                        if did_flash_this_step.contains(&(x_neighbour, y_neighbour)) {
                            continue;
                        }
                        *next_to_a_flash
                            .entry((x_neighbour, y_neighbour))
                            .or_insert(0) += 1;
                    }
                }
            }
            for ((x, y), flashes_exposed_to) in &next_to_a_flash {
                if let Some(light_level) = grid.grid.get_mut(&(*x, *y)) {
                    *light_level += flashes_exposed_to;
                }
            }
            if next_to_a_flash.is_empty() {
                // things have died down, time to do next step
                break 'inner;
            }
        }
        // everything has died down, prepare for new step
        for (x, y) in did_flash_this_step {
            grid.grid.insert((x, y), 0);
        }
    }

    flashes
}

#[aoc(day11, part2)]
fn part2(grid: &Grid<u8>) -> usize {
    let mut grid = grid.clone();

    let number_of_octopii = grid.grid.len();
    for step in 1..=10000 {
        // first increase all values by 1
        for ((_x, _y), energy_level) in &mut grid.grid {
            *energy_level += 1;
        }
        //flash
        let mut did_flash_this_step = HashSet::new();
        'inner: loop {
            let mut next_to_a_flash = HashMap::new();
            for ((x, y), energy_level) in &mut grid.grid {
                if did_flash_this_step.contains(&(*x, *y)) {
                    continue;
                }
                if *energy_level > 9 {
                    did_flash_this_step.insert((*x, *y));
                    for (x_neighbour, y_neighbour) in get_eight_neighbours(*x, *y) {
                        if did_flash_this_step.contains(&(x_neighbour, y_neighbour)) {
                            continue;
                        }
                        *next_to_a_flash
                            .entry((x_neighbour, y_neighbour))
                            .or_insert(0) += 1;
                    }
                }
            }
            for ((x, y), flashes_exposed_to) in &next_to_a_flash {
                if let Some(light_level) = grid.grid.get_mut(&(*x, *y)) {
                    *light_level += flashes_exposed_to;
                }
            }
            if next_to_a_flash.is_empty() {
                // things have died down, time to do next step
                break 'inner;
            }
        }
        // everything has died down, prepare for new step
        for (x, y) in &did_flash_this_step {
            grid.grid.insert((*x, *y), 0);
        }
        if number_of_octopii == did_flash_this_step.len() {
            return step;
        }
    }
    panic!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn verify_part1() {
        let input = include_str!("../input/2021/day11.txt");
        let input = parse_input(input);
        assert_eq!(part1(&input), 1691);
    }

    #[test]
    fn verify_part2() {
        let input = include_str!("../input/2021/day11.txt");
        assert_eq!(part2(&parse_input(input)), 216);
    }

    #[test]
    fn part1_provided_example() {
        let result = part1(&parse_input(
            r#"5483143223
2745854711
5264556173
6141336146
6357385478
4167524645
2176841721
6882881134
4846848554
5283751526"#,
        ));

        assert_eq!(result, 1656)
    }

    #[test]
    fn part2_provided_example() {
        let result = part2(&parse_input(
            r#"5483143223
2745854711
5264556173
6141336146
6357385478
4167524645
2176841721
6882881134
4846848554
5283751526"#,
        ));

        assert_eq!(result, 195)
    }
}
