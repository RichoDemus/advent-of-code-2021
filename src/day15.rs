use crate::grid::Grid;
use std::collections::HashMap;

#[aoc_generator(day15)]
fn parse_input(input: &str) -> Vec<Vec<u32>> {
    input
        .lines()
        .map(|line| line.chars().map(|c| c.to_digit(10).unwrap()).collect())
        .collect()
}

// just recursively find all paths, stop when at end or when steps > matrix.size
// added so that we also remembered the nodes we've visited so we don't revisit them
// added a "global lowest path" updates once we find an exit, abandon paths if > global lowest
// begin by finding a naive GLP by just zig-zagging to the end
// none of them worked

#[aoc(day15, part1, djikstra)]
fn part1(input: &[Vec<u32>]) -> u32 {
    let cave = input.to_vec();
    find_least_risky_path(cave)
}

fn find_least_risky_path(mut cave: Vec<Vec<u32>>) -> u32 {
    cave[0][0] = 0;
    let startx: i32 = 0;
    let starty: i32 = 0;

    let endx: i32 = cave.len() as i32 - 1;
    let endy: i32 = cave.len() as i32 - 1;
    println!(
        "Want to move from {},{} to {},{}",
        startx, starty, endx, endy
    );

    let result = pathfinding::prelude::dijkstra(
        &(startx, starty),
        |(x, y)| {
            let mut neighbours = vec![];
            for (nx, ny) in vec![(*x + 1, *y), (*x, *y + 1), (*x - 1, *y), (*x, y - 1)] {
                if nx < 0 {
                    continue;
                }
                if ny < 0 {
                    continue;
                }
                if ny > endy {
                    continue;
                }
                if nx > endx {
                    continue;
                }

                let value = cave[ny as usize][nx as usize];
                neighbours.push(((nx, ny), value));
            }
            neighbours
        },
        |(x, y)| *x == endx && *y == endy,
    );

    return result.unwrap().1;
}

#[aoc(day15, part2, djikstra)]
fn part2(input: &[Vec<u32>]) -> u32 {
    let cave = input.to_vec();
    let mut cave = expand_grid_wrapper(cave);

    cave[0][0] = 0;
    return find_least_risky_path(cave);
}

// todo revamp this to not convert to and from grid
fn expand_grid_wrapper(input: Vec<Vec<u32>>) -> Vec<Vec<u32>> {
    let mut cave_map = HashMap::new();
    for (y, row) in input.iter().enumerate() {
        for (x, cell) in row.iter().enumerate() {
            cave_map.insert((x as i64, y as i64), *cell as u8);
        }
    }

    let grid = Grid { grid: cave_map };
    let grid = expand_grid2(grid);

    // convert to matrix
    let x_min = *grid.grid.keys().map(|(x, _y)| x).min().unwrap();
    let x_max = *grid.grid.keys().map(|(x, _y)| x).max().unwrap();
    let y_min = *grid.grid.keys().map(|(_x, y)| y).min().unwrap();
    let y_max = *grid.grid.keys().map(|(_x, y)| y).max().unwrap();

    let mut matrix = vec![];
    for y in y_min..=y_max {
        let mut row = vec![];
        for x in x_min..=x_max {
            row.push(*grid.grid.get(&(x, y)).unwrap() as u32);
        }
        matrix.push(row);
    }

    matrix
}

fn expand_grid2(mut expand_grid: Grid<u8>) -> Grid<u8> {
    let x_width = expand_grid.x_max() as u32 + 1;
    let y_width = expand_grid.y_max() as u32 + 1;

    let mut new_nodes = HashMap::new();
    for board_x in 0..=4 {
        for board_y in 0..=4 {
            for ((gx, gy), value) in expand_grid.grid.iter() {
                let value = *value;
                let mut value = value + board_x + board_y;
                while value > 9 {
                    value -= 9;
                }
                let new_x = *gx as u32 + board_x as u32 * x_width;
                let new_y = *gy as u32 + board_y as u32 * y_width;
                new_nodes.insert((new_x, new_y), value);
            }
        }
    }

    for ((x, y), v) in new_nodes {
        expand_grid.grid.insert((x.into(), y.into()), v);
    }
    expand_grid
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn verify_part1() {
        let input = include_str!("../input/2021/day15.txt");
        let input = parse_input(input);
        assert_eq!(part1(input.as_slice()), 583);
    }

    #[test]
    fn verify_part2() {
        let input = include_str!("../input/2021/day15.txt");
        assert_eq!(part2(&parse_input(input)), 2927);
    }

    #[test]
    fn part1_provided_example() {
        let input = parse_input(
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
        );

        assert_eq!(part1(input.as_slice()), 40);
    }

    #[test]
    fn test_small_example() {
        let input = parse_input(
            r#"1111
8211
8199
1171"#,
        );

        assert_eq!(part1(input.as_slice()), 13);
    }

    #[test]
    fn part2_provided_example() {
        let input = parse_input(
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
        );

        assert_eq!(part2(input.as_slice()), 315);
    }
}
