use std::ops::Not;
use crate::grid::{get_four_neighbours, Grid};

#[aoc_generator(day15)]
fn parse_input(input: &str) -> Grid<u8>{
    Grid::from_non_delim_block(input)
}


#[aoc(day15, part1)]
fn part1(input: &Grid<u8>) -> u64 {
    println!("{}", input.to_string());
    let startx = 0;
    let starty = 0;

    let endx = input.x_max();
    let endy = input.y_max();

    calc_lowest_risk_factor(input, startx, starty, endx, endy, 0, 0).unwrap()
}

fn calc_lowest_risk_factor(grid: &Grid<u8>, x: i64, y: i64, x_goal: i64, y_goal: i64, steps: u64, mut current_risk_factor: u64) -> Option<u64> {
    if steps > grid.size() {
        return None;
    }

    if grid.is_within_bounds(x,y).not() {
        return None;
    }

    // we're at a valid position
    if !(x == 0 && y == 0) {
        current_risk_factor += *grid.grid.get(&(x,y)).unwrap() as u64;
    }

    if x == x_goal && y == y_goal {
        return Some(current_risk_factor);
    }

    // lets go in all 4 directions, becaues lazy
    let mut lowest_risk_factor = None;
    for (next_x, next_y) in vec![(x, y+1), (x+1,y)] {
        let new_risk_factor = calc_lowest_risk_factor(grid, next_x, next_y, x_goal, y_goal, steps +1, current_risk_factor);
        lowest_risk_factor = match (lowest_risk_factor, new_risk_factor) {
            (None, None) => None,
            (None, Some(risk)) => Some(risk),
            (Some(old), Some(new)) if new < old => Some(new),
            (Some(risk), None) => Some(risk),
            (Some(risk), Some(_higher_risk)) => Some(risk),
        };
    }
    lowest_risk_factor
}

// #[aoc(day15, part2)]
// fn part2(input: &[Line]) -> usize {
//     todo!()
// }

#[cfg(test)]
mod tests {
    use super::*;

// #[test]
    // fn verify_part1() {
    //     let input = include_str!("../input/2021/day15.txt");
    //     let input = parse_input(input);
    //     assert_eq!(part1(input.as_slice()), 6666);
    // }

    // #[test]
    // fn verify_part2() {
    //     let input = include_str!("../input/2021/day15.txt");
    //     assert_eq!(part2(parse_input(input).as_slice()), 19081);
    // }

    #[test]
    fn part1_provided_example() {
        let result = part1(&parse_input(
            r#"1163751742
1381373672
2136511328
3694931569
7463417111
1319128137
1359912421
3125421639
1293138521
2311944581"#,
        ));

        assert_eq!(result, 40)
    }

    #[test]
    fn part2_provided_example() {}
}
