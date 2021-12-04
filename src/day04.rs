use std::collections::VecDeque;
use std::ops::Not;
use split_iter::Splittable;

#[aoc_generator(day4)]
fn parse_input(input: &str) -> Bingo {
    let mut lines = input.lines();
    let numbers = lines.next().unwrap().split(",").map(|s|s.parse().expect("parse int")).collect::<Vec<_>>();

    let mut boards = vec![];
    let mut buffer = vec![];
    while let Some(line) = lines.next() {
        if line.is_empty() {
            continue
        }
        println!("parsing {}", line);
        let numbers = line.split_ascii_whitespace()
            .map(|s|s.parse().expect("parse int2"))
            .map(|number|Cell{number, marked: false})
            .collect::<Vec<_>>();
        buffer.push(numbers);
        if buffer.len() == 5 {
            let numbers = std::mem::replace(&mut buffer, vec![]);
            boards.push(Board {
                cells: numbers,
                done:false,
            })
        }
    }
    Bingo {
        numbers,
        boards
    }
}

#[derive(Debug, Clone)]
struct Bingo {
    numbers: Vec<u64>,
    boards: Vec<Board>,
}
#[derive(Debug, Clone)]
struct Board {
    cells: Vec<Vec<Cell>>,
    done: bool,
}
impl Board {
    fn mark(&mut self, number: u64) {
        for mut row in &mut self.cells {
            for mut cell in row {
                if cell.number == number {
                    cell.marked = true;
                }
            }
        }
    }
    fn bingo(&self) -> bool {
        // check rows
        for row in &self.cells {
            if row.iter().all(|cell|cell.marked) {
                return true
            }
        }
        // check columns
        for column in 0..5 {
            if self.cells[0][column].marked && self.cells[1][column].marked && self.cells[2][column].marked && self.cells[3][column].marked && self.cells[4][column].marked {
                return true
            }
        }

        false
    }
}
#[derive(Debug, Clone)]
struct Cell {
    number: u64,
    marked: bool,
}

#[aoc(day4, part2)]
fn part2(input: &Bingo) -> u64 {
    let numbers = input.numbers.clone();
    let mut numbers = VecDeque::from(numbers);
    let mut boards = input.boards.clone();
    let mut boards_left = boards.len();

    while let Some(number) = numbers.pop_front() {
        for mut board in &mut boards {
            board.mark(number);
            if board.bingo() {
                if board.done.not() {
                    boards_left -= 1;
                    board.done = true;
                }
                if boards_left == 0 {
                    let unmarked_sum:u64 = board.cells.iter().flat_map(|row|row.iter())
                        .filter(|cell|cell.marked == false)
                        .map(|cell|cell.number)
                        .sum();
                    return unmarked_sum * number;
                }
            }
        }
    }

    panic!("Unreachable code")
}

#[aoc(day4, part1)]
fn part1(input: &Bingo) -> u64 {
    let numbers = input.numbers.clone();
    let mut numbers = VecDeque::from(numbers);
    let mut boards = input.boards.clone();

    while let Some(number) = numbers.pop_front() {
        for mut board in &mut boards {
            board.mark(number);
            if board.bingo() {
                let unmarked_sum:u64 = board.cells.iter().flat_map(|row|row.iter())
                    .filter(|cell|cell.marked == false)
                    .map(|cell|cell.number)
                    .sum();
                return unmarked_sum * number;
            }
        }
    }

    panic!("Unreachable code")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn verify_part1() {
        let input = include_str!("../input/2021/day4.txt");
        let input = parse_input(input);
        assert_eq!(part1(&input), 25410);
    }

    #[test]
    fn verify_part2() {
        let input = include_str!("../input/2021/day4.txt");
        assert_eq!(part2(&parse_input(input)), 2730);
    }

    #[test]
    fn part1_provided_example() {
        let result = part1(&parse_input(
            r#"7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

22 13 17 11  0
 8  2 23  4 24
21  9 14 16  7
 6 10  3 18  5
 1 12 20 15 19

 3 15  0  2 22
 9 18 13 17  5
19  8  7 25 23
20 11 10 24  4
14 21 16 12  6

14 21 17 24  4
10 16 15  9 19
18  8 23 26 20
22 11 13  6  5
 2  0 12  3  7"#,
        ));

        assert_eq!(result, 4512)
    }

    #[test]
    fn part2_provided_example() {
        let result = part2(&parse_input(
            r#"7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

22 13 17 11  0
 8  2 23  4 24
21  9 14 16  7
 6 10  3 18  5
 1 12 20 15 19

 3 15  0  2 22
 9 18 13 17  5
19  8  7 25 23
20 11 10 24  4
14 21 16 12  6

14 21 17 24  4
10 16 15  9 19
18  8 23 26 20
22 11 13  6  5
 2  0 12  3  7"#,
        ));

        assert_eq!(result, 1924)
    }
}
