use std::collections::{HashMap, HashSet};
use std::ops::Not;
use std::sync::Mutex;
use crate::grid::{get_four_neighbours, Grid};

#[aoc_generator(day15)]
fn parse_input(input: &str) -> Grid<u8>{
    Grid::from_non_delim_block(input)
}


// #[aoc(day15, part1)]
fn part1(input: &Grid<u8>) -> u64 {
    let mut grid = input.clone();
    // nice hack
    grid.grid.insert((0,0), 0);
    println!("{}", input.to_string());
    let startx = 0;
    let starty = 0;

    let endx = input.x_max();
    let endy = input.y_max();

    calc_lowest_risk_factor(&grid, startx, starty, endx, endy, 0, 0).unwrap()
}

fn calc_lowest_risk_factor(grid: &Grid<u8>, x: i64, y: i64, x_goal: i64, y_goal: i64, steps: u64, mut current_risk_factor: u64) -> Option<u64> {
    if steps > grid.size() {
        return None;
    }

    if grid.is_within_bounds(x,y).not() {
        return None;
    }

    // we're at a valid position
    current_risk_factor += *grid.grid.get(&(x,y)).unwrap() as u64;

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

fn calc_shortest_path_naive(grid: &Grid<u8>, mut x: i64, mut y: i64, x_goal: i64, y_goal: i64) -> u64 {
    let mut risk_factor = 0;
    loop {
        let risk = *grid.grid.get(&(x, y)).unwrap() as u64;
        risk_factor += risk;
        // println!("{},{} -> {} is part of the naive path",x,y,risk);

        let can_increase_x = grid.is_within_bounds(x+1, y);
        let can_increase_y = grid.is_within_bounds(x, y+1);
        // println!("\t{} {}",can_increase_x,can_increase_y);

        if can_increase_x && can_increase_y {
            if x > y{
                y += 1;
            } else {
                x += 1;
            }
        } else if can_increase_x {
            x += 1;
        } else if can_increase_y {
            y += 1;
        } else if x == x_goal && y == y_goal {
            break;
        } else {
            panic!()
        }

    }
    risk_factor
}

// #[aoc(day15, part1,path)]
fn part1_save_path(input: &Grid<u8>) -> u64 {
    let mut grid = input.clone();
    // nice hack
    grid.grid.insert((0,0), 0);
    // println!("{}", input.to_string());
    let startx = 0;
    let starty = 0;

    let endx = input.x_max();
    let endy = input.y_max();
    println!("goal: {},{}", endx, endy);

    let naive_score = calc_shortest_path_naive(&grid, startx, starty, endx, endy);
    let naive_score = Mutex::new(naive_score);
    println!("Naive path: {}", naive_score.lock().unwrap());

    calc_lowest_risk_factor_path(&grid, startx, starty, endx, endy, HashSet::new(), 0, &naive_score).unwrap()
}

fn calc_lowest_risk_factor_path(grid: &Grid<u8>, x: i64, y: i64, x_goal: i64, y_goal: i64, mut path: HashSet<(i64,i64)>, mut current_risk_factor: u64, score_to_beat: &Mutex<u64>) -> Option<u64> {
    if path.contains(&(x,y)) {
        return None;
    }

    if grid.is_within_bounds(x,y).not() {
        return None;
    }

    // we're at a valid position
    current_risk_factor += *grid.grid.get(&(x,y)).unwrap() as u64;
    if current_risk_factor > *score_to_beat.lock().unwrap() {
        return None;
    }
    path.insert((x,y));
    // println!("path: {:?}", path);

    if x == x_goal && y == y_goal {
        println!("Found one exit: {}", current_risk_factor);
        {
            let current_record = *score_to_beat.lock().unwrap();
            *score_to_beat.lock().unwrap() = current_record.min(current_risk_factor);
        }
        // println!("Current risk factor is now {}", score_to_beat.lock().unwrap());
        return Some(current_risk_factor);
    }

    // lets go in all 4 directions, becaues lazy
    let mut lowest_risk_factor = None;
    for (next_x, next_y) in get_four_neighbours(x,y) {
        if path.contains(&(next_x, next_y)) {
            // we've already been there before
            continue
        }

        let new_risk_factor = calc_lowest_risk_factor_path(grid, next_x, next_y, x_goal, y_goal, path.clone(), current_risk_factor, score_to_beat);
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

// #[aoc(day15, part1,path2)]
fn part1_save_path2(input: &Grid<u8>) -> u64 {
    let mut grid = input.clone();
    // nice hack
    grid.grid.insert((0,0), 0);
    // println!("{}", input.to_string());
    let startx = 0;
    let starty = 0;

    let endx = input.x_max();
    let endy = input.y_max();
    println!("goal: {},{} -> {:?}", endx, endy, grid.grid.get(&(endx,endy)));

    let naive_score = calc_shortest_path_naive(&grid, startx, starty, endx, endy);
    let naive_score = Mutex::new(naive_score);
    println!("Naive path: {}", naive_score.lock().unwrap());

    calc_lowest_risk_factor_path2(&grid, startx, starty, endx, endy, 0, &naive_score).unwrap()
}

fn calc_lowest_risk_factor_path2(grid: &Grid<u8>, x: i64, y: i64, x_goal: i64, y_goal: i64, mut current_risk_factor: u64, score_to_beat: &Mutex<u64>) -> Option<u64> {
    if grid.is_within_bounds(x,y).not() {
        return None;
    }

    // we're at a valid position
    current_risk_factor += *grid.grid.get(&(x,y)).unwrap() as u64;
    if current_risk_factor > *score_to_beat.lock().unwrap() {
        return None;
    }
    // println!("path: {:?}", path);

    if x == x_goal && y == y_goal {
        println!("Found one exit: {}", current_risk_factor);
        {
            let current_record = *score_to_beat.lock().unwrap();
            *score_to_beat.lock().unwrap() = current_record.min(current_risk_factor);
        }
        // println!("Current risk factor is now {}", score_to_beat.lock().unwrap());
        return Some(current_risk_factor);
    }

    // lets go in all 4 directions, becaues lazy
    let mut lowest_risk_factor = None;
    // for (next_x, next_y) in get_four_neighbours(x,y) {
    for (next_x, next_y) in vec![(x+1,y),(x,y+1)] {

        let new_risk_factor = calc_lowest_risk_factor_path2(grid, next_x, next_y, x_goal, y_goal, current_risk_factor, score_to_beat);
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

#[aoc(day15, part1,djikstra)]
fn part1_save_djikstra(input: &Grid<u8>) -> i64 {
    let mut grid = input.clone();
    // nice hack
    grid.grid.insert((0,0), 0);
    // println!("{}", input.to_string());
    let startx:i64 = 0;
    let starty:i64 = 0;

    let endx = input.x_max();
    let endy = input.y_max();
    println!("goal: {},{} -> {:?}", endx, endy, grid.grid.get(&(endx,endy)));

    let result = pathfinding::prelude::dijkstra(
        &(startx, starty),
        |(x,y)| {
            grid.calc_in_bounds_four_way_neighbours(*x, *y).into_iter().map(|(x,y)| {
                ((x,y), *grid.grid.get(&(x,y)).unwrap() as i64)
            })
    },
    // |(x,y)|{
    //     grid.grid.get(&(*x,*y)).unwrap() as i64
    // },
    |(x,y)|{
        *x == endx && *y == endy
    });


    println!("{:?}", result);

    return result.unwrap().1;

    todo!()
}

#[aoc(day15, part2,djikstra)]
fn part2_save_djikstra(input: &Grid<u8>) -> i64 {
    let mut grid = input.clone();
    // nice hack

    let startx:i64 = 0;
    let starty:i64 = 0;

    let mut grid = expand_grid(grid);
    // println!("{}", grid.to_string());
    grid.grid.insert((0,0), 0);

    let endx = grid.x_max();
    let endy = grid.y_max();
    println!("goal: {},{} -> {:?}", endx, endy, grid.grid.get(&(endx,endy)));
let endx = endx as usize;
    let endy = endy as usize;

    // convert to matrix
    let x_min = *grid.grid.keys().map(|(x, _y)| x).min().unwrap();
    let x_max = *grid.grid.keys().map(|(x, _y)| x).max().unwrap();
    let y_min = *grid.grid.keys().map(|(_x, y)| y).min().unwrap();
    let y_max = *grid.grid.keys().map(|(_x, y)| y).max().unwrap();

    let mut matrix = vec![];
    for y in y_min..=y_max {
        let mut row = vec![];
        for x in x_min..=x_max {
            row.push(*grid.grid.get(&(x,y)).unwrap() as i64)
        }
        matrix.push(row);
    }

    // for row in matrix {
    //     for cell in row {
    //         print!("{}", cell);
    //     }
    //     println!()
    // }
    // panic!();

    

    let result = pathfinding::prelude::dijkstra(
        &(startx, starty),
        |(x,y)| {
            let mut neighbours = vec![];
            for (nx, ny) in vec![(*x+1,*y),(*x,*y+1),(*x-1,*y),(*x,y-1)] {
                if nx < 0 {
                    continue
                }
                if ny < 0 {
                    continue
                }
                if ny >= matrix.len() as i64 {
                    continue
                }
                if nx >= matrix.len() as i64 {
                    continue
                }

                let value = matrix[ny as usize][nx as usize];
                neighbours.push(((nx,ny), value));
            }
            neighbours
        },
        |(x,y)|{
            *x as usize == endx && *y as usize == endy
        });


    println!("{:?}", result);

    return result.unwrap().1;

    todo!()
}

fn expand_grid(mut expand_grid: Grid<u8>) -> Grid<u8> {
    let x_width = expand_grid.x_max() + 1;
    let y_width = expand_grid.y_max() + 1;

    let mut new_nodes = HashMap::new();
    for board_x in 0..=4 {
        for board_y in 0..=4 {
            for ((gx,gy), value) in expand_grid.grid.iter() {
                // println!("convert {},{} => {}", gx, gy, value);
                let value = *value as i64;
                let mut value = value as i64 + board_x + board_y;
                while value > 9 {
                    value -= 9;
                }
                let new_x = gx + board_x * x_width;
                let new_y = gy + board_y * y_width;
                new_nodes.insert((new_x, new_y), value);
                // println!("\tinsert {},{}={}", new_x, new_y, value);
            }
        }
    }

    for ((x,y),v) in new_nodes {
        expand_grid.grid.insert((x,y),v as u8);
    }
    expand_grid
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

        assert_eq!(part1(&input), 40);
    }

    #[test]
    fn part1_provided_example_path() {
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

        assert_eq!(part1_save_path(&input), 40);
    }

    #[test]
    fn part1_provided_example_path2() {
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

        assert_eq!(part1_save_path2(&input), 40);
    }

    #[test]
    fn part1_provided_example_djikstra() {
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

        assert_eq!(part1_save_djikstra(&input), 40);
    }

    #[test]
    fn test_small_example() {
        let input = parse_input(
            r#"1111
8211
8199
1171"#,
        );

        assert_eq!(part1_save_path(&input), 13);
    }

    #[test]
    fn test_small_example_astar() {
        let input = parse_input(
            r#"1111
8211
8199
1171"#,
        );

        assert_eq!(part1_save_djikstra(&input), 13);
    }

    #[test]
    fn test_small_example_part2_djikstra() {
        let input = parse_input(
            r#"12
34"#,
        );

        assert_eq!(part2_save_djikstra(&input), 13);
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

        assert_eq!(part2_save_djikstra(&input), 315);
    }

}
