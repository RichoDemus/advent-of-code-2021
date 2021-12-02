use std::str::FromStr;

use strum::EnumString;

#[aoc_generator(day2)]
fn parse_input(input: &str) -> Vec<Command> {
    input.lines().map(|line| line.parse().unwrap()).collect()
}

#[derive(Eq, PartialEq, EnumString, Debug)]
#[strum(ascii_case_insensitive)]
enum Direction {
    Forward,
    Up,
    Down,
}

#[derive(Eq, PartialEq, Debug)]
struct Command {
    direction: Direction,
    length: u64,
}

impl FromStr for Command {
    type Err = String;

    // todo don't panic
    fn from_str(line: &str) -> Result<Self, Self::Err> {
        let mut tokens = line.split_ascii_whitespace();
        let direction = tokens.next().ok_or("Couldn't extract direction")?;
        let direction: Direction = direction
            .parse()
            .map_err(|e| format!("Unable to parse direction: {:?}", e))?;
        let length = tokens
            .next()
            .ok_or("Couldn't extract direction")?
            .parse()
            .map_err(|e| format!("Unable to parse int: {:?}", e))?;
        Ok(Self { direction, length })
    }
}

#[aoc(day2, part1)]
fn part1(commands: &[Command]) -> u64 {
    let mut horizontal_position = 0;
    let mut depth: u64 = 0;

    for command in commands {
        match command {
            Command {
                direction: Direction::Forward,
                length,
            } => horizontal_position += length,
            Command {
                direction: Direction::Up,
                length,
            } => depth = depth.saturating_sub(*length),
            Command {
                direction: Direction::Down,
                length,
            } => depth = depth.saturating_add(*length),
        }
    }
    horizontal_position * depth
}

#[aoc(day2, part2)]
fn part2(commands: &[Command]) -> u64 {
    let mut horizontal_position = 0;
    let mut depth: u64 = 0;
    let mut aim = 0;

    for command in commands {
        match command {
            Command {
                direction: Direction::Forward,
                length,
            } => {
                horizontal_position += length;
                depth = depth.saturating_add(aim * length);
            }
            Command {
                direction: Direction::Up,
                length,
            } => aim = aim.saturating_sub(*length),
            Command {
                direction: Direction::Down,
                length,
            } => aim = aim.saturating_add(*length),
        }
    }
    horizontal_position * depth
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn verify_part1() {
        let input = include_str!("../input/2021/day2.txt");
        let input = parse_input(input);
        assert_eq!(part1(input.as_slice()), 2117664);
    }

    #[test]
    fn verify_part2() {
        let input = include_str!("../input/2021/day2.txt");
        assert_eq!(part2(parse_input(input).as_slice()), 2073416724);
    }

    #[test]
    fn test_parse() {
        let input = r#"forward 5
down 5
forward 8
up 3
down 8
forward 2"#;

        let parsed = parse_input(input);

        assert_eq!(
            parsed,
            vec![
                Command {
                    direction: Direction::Forward,
                    length: 5,
                },
                Command {
                    direction: Direction::Down,
                    length: 5,
                },
                Command {
                    direction: Direction::Forward,
                    length: 8,
                },
                Command {
                    direction: Direction::Up,
                    length: 3,
                },
                Command {
                    direction: Direction::Down,
                    length: 8,
                },
                Command {
                    direction: Direction::Forward,
                    length: 2,
                },
            ]
        );
    }

    #[test]
    fn part1_provided_example() {
        let result = part1(&[
            Command {
                direction: Direction::Forward,
                length: 5,
            },
            Command {
                direction: Direction::Down,
                length: 5,
            },
            Command {
                direction: Direction::Forward,
                length: 8,
            },
            Command {
                direction: Direction::Up,
                length: 3,
            },
            Command {
                direction: Direction::Down,
                length: 8,
            },
            Command {
                direction: Direction::Forward,
                length: 2,
            },
        ]);
        assert_eq!(result, 150);
    }

    #[test]
    fn part2_provided_example() {
        let result = part2(&[
            Command {
                direction: Direction::Forward,
                length: 5,
            },
            Command {
                direction: Direction::Down,
                length: 5,
            },
            Command {
                direction: Direction::Forward,
                length: 8,
            },
            Command {
                direction: Direction::Up,
                length: 3,
            },
            Command {
                direction: Direction::Down,
                length: 8,
            },
            Command {
                direction: Direction::Forward,
                length: 2,
            },
        ]);
        assert_eq!(result, 900);
    }
}
