use std::collections::HashMap;
use std::ops::Not;

use once_cell::unsync::Lazy;
use regex::Regex;

#[aoc_generator(day5)]
fn parse_input(input: &str) -> Vec<Line> {
    let regex = Lazy::new(|| Regex::new(r"(\d*),(\d*)\s*->\s(\d*),(\d*)").unwrap());
    regex.captures_iter(input)
        .map(|groups| Line {
            x1: groups[1].parse().unwrap(),
            y1: groups[2].parse().unwrap(),
            x2: groups[3].parse().unwrap(),
            y2: groups[4].parse().unwrap(),
        })
        .collect()
}

#[derive(Eq, PartialEq, Debug)]
struct Line {
    x1: i64,
    y1: i64,
    x2: i64,
    y2: i64,
}

impl Line {
    fn points(&self) -> Vec<(i64, i64)> {
        let mut result = vec![];
        let x_direction_modifier = if self.x1 < self.x2 {
            1
        } else if self.x1 == self.x2 {
            0
        } else {
            -1
        };

        let y_direction_modifier = if self.y1 < self.y2 {
            1
        } else if self.y1 == self.y2 {
            0
        } else {
            -1
        };

        //start is x1, y1. end is x2 y2
        let x_steps = self.x1.max(self.x2) - self.x1.min(self.x2);
        let y_steps = self.y1.max(self.y2) - self.y1.min(self.y2);
        let steps = x_steps.max(y_steps);
        for step in 0..=steps {
            let new_point = (
                self.x1 + step * x_direction_modifier,
                self.y1 + step * y_direction_modifier,
            );
            result.push(new_point);
        }
        result
    }
    fn is_straight(&self) -> bool {
        let x_min = self.x1.min(self.x2);
        let x_max = self.x1.max(self.x2);
        let y_min = self.y1.min(self.y2);
        let y_max = self.y1.max(self.y2);
        (x_min == x_max) || (y_min == y_max)
    }
}

#[aoc(day5, part1)]
fn part1(input: &[Line]) -> usize {
    let mut floor = HashMap::new();
    for line in input {
        if line.is_straight().not() {
            continue;
        }
        for point in line.points() {
            *floor.entry(point).or_insert(0) += 1;
        }
    }

    floor.values().filter(|value| value > &&1).count()
}

#[aoc(day5, part2)]
fn part2(input: &[Line]) -> usize {
    let mut floor = HashMap::new();
    for line in input {
        for point in line.points() {
            *floor.entry(point).or_insert(0) += 1;
        }
    }

    floor.values().filter(|value| value > &&1).count()
}

#[allow(dead_code)]
fn print(seats: &HashMap<(i64, i64), i32>) {
    let x_min = *seats.keys().map(|(x, _)| x).min().unwrap();
    let x_max = *seats.keys().map(|(x, _)| x).max().unwrap();
    let y_min = *seats.keys().map(|(_, y)| y).min().unwrap();
    let y_max = *seats.keys().map(|(_, y)| y).max().unwrap();
    for y in y_min..=y_max {
        for x in x_min..=x_max {
            match seats.get(&(x, y)) {
                None => print!("."),
                Some(amount) => print!("{}", amount),
            }
        }
        println!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn verify_part1() {
        let input = include_str!("../input/2021/day5.txt");
        let input = parse_input(input);
        assert_eq!(part1(input.as_slice()), 6666);
    }

    #[test]
    fn verify_part2() {
        let input = include_str!("../input/2021/day5.txt");
        assert_eq!(part2(parse_input(input).as_slice()), 19081);
    }

    #[test]
    fn part1_provided_example() {
        let result = part1(&parse_input(
            r#"0,9 -> 5,9
8,0 -> 0,8
9,4 -> 3,4
2,2 -> 2,1
7,0 -> 7,4
6,4 -> 2,0
0,9 -> 2,9
3,4 -> 1,4
0,0 -> 8,8
5,5 -> 8,2"#,
        ));

        assert_eq!(result, 5)
    }

    #[test]
    fn part2_provided_example() {
        let result = part2(&parse_input(
            r#"0,9 -> 5,9
8,0 -> 0,8
9,4 -> 3,4
2,2 -> 2,1
7,0 -> 7,4
6,4 -> 2,0
0,9 -> 2,9
3,4 -> 1,4
0,0 -> 8,8
5,5 -> 8,2"#,
        ));

        assert_eq!(result, 12)
    }
}
