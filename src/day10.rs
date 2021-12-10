use std::collections::VecDeque;

use itertools::Itertools;

#[aoc_generator(day10)]
fn parse_input(input: &str) -> Vec<String> {
    input
        .lines()
        .map(std::string::ToString::to_string)
        .collect()
}

#[aoc(day10, part1)]
fn part1(input: &[String]) -> usize {
    let opening_chars = vec!['(', '[', '{', '<'];
    let closing_chars = vec![')', ']', '}', '>'];
    input
        .iter()
        .enumerate()
        .map(|(_i, line)| {
            let mut stack = VecDeque::new();
            for (_j, char) in line.chars().enumerate() {
                if opening_chars.contains(&char) {
                    stack.push_front(char);
                } else if closing_chars.contains(&char) {
                    let popped = match stack.pop_front() {
                        None => panic!("invalid"),
                        Some(p) => p,
                    };
                    let ok = match popped {
                        '(' if char == ')' => true,
                        '[' if char == ']' => true,
                        '{' if char == '}' => true,
                        '<' if char == '>' => true,
                        _ => false,
                    };
                    if !ok {
                        let score = match char {
                            ')' => 3,
                            ']' => 57,
                            '}' => 1197,
                            '>' => 25137,
                            _ => todo!(),
                        };
                        // println!("{} line {:?} illegal {} (expected {}) at {} for a score of {}", i, line, char, popped, j, score);
                        return score;
                    }
                }
            }
            0
        })
        .sum()
}

#[aoc(day10, part1, looperino)]
fn part1_loop(input: &[String]) -> usize {
    let closing_chars = vec![')', ']', '}', '>'];
    input
        .iter()
        .map(|l| {
            let mut line = l.clone();
            loop {
                let length = line.len();
                line = line.replace("()", "");
                line = line.replace("[]", "");
                line = line.replace("{}", "");
                line = line.replace("<>", "");
                if line.len() == length {
                    break;
                }
            }
            if let Some(char) = line.chars().find(|c| closing_chars.contains(c)) {
                match char {
                    ')' => 3,
                    ']' => 57,
                    '}' => 1197,
                    '>' => 25137,
                    _ => todo!(),
                }
            } else {
                0
            }
        })
        .sum()
}

#[aoc(day10, part2)]
fn part2(input: &[String]) -> usize {
    let opening_chars = vec!['(', '[', '{', '<'];
    let closing_chars = vec![')', ']', '}', '>'];
    let scores = input
        .iter()
        .enumerate()
        .map(|(_i, line)| {
            let mut stack = VecDeque::new();
            for (_j, char) in line.chars().enumerate() {
                if opening_chars.contains(&char) {
                    stack.push_front(char);
                } else if closing_chars.contains(&char) {
                    let popped = match stack.pop_front() {
                        None => panic!("invalid"),
                        Some(p) => p,
                    };
                    let ok = match popped {
                        '(' if char == ')' => true,
                        '[' if char == ']' => true,
                        '{' if char == '}' => true,
                        '<' if char == '>' => true,
                        _ => false,
                    };
                    if !ok {
                        return 0; // lets just discard the corrupted ones
                    }
                }
            }
            // no errors, but we might have tokens left on the stack
            let mut score = 0;

            for missing in stack {
                let points = match missing {
                    '(' => 1,
                    '[' => 2,
                    '{' => 3,
                    '<' => 4,
                    e => panic!("unexpected char: {}", e),
                };
                score *= 5;
                score += points;
            }

            score
        })
        .filter(|score| *score > 0)
        .sorted()
        .collect::<Vec<_>>();
    scores[scores.len() / 2]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn verify_part1() {
        let input = include_str!("../input/2021/day10.txt");
        let input = parse_input(input);
        assert_eq!(part1(input.as_slice()), 311895);
        assert_eq!(part1_loop(input.as_slice()), 311895);
    }

    #[test]
    fn verify_part2() {
        let input = include_str!("../input/2021/day10.txt");
        assert_eq!(part2(parse_input(input).as_slice()), 2904180541);
    }

    #[test]
    fn part1_provided_example() {
        let input = parse_input(
            r#"[({(<(())[]>[[{[]{<()<>>
[(()[<>])]({[<{<<[]>>(
{([(<{}[<>[]}>{[]{[(<()>
(((({<>}<{<{<>}{[]{[]{}
[[<[([]))<([[{}[[()]]]
[{[{({}]{}}([{[{{{}}([]
{<[[]]>}<{[{[{[]{()[[[]
[<(<(<(<{}))><([]([]()
<{([([[(<>()){}]>(<<{{
<{([{{}}[<[[[<>{}]]]>[]]"#,
        );

        assert_eq!(part1(&input), 26397);
        assert_eq!(part1_loop(&input), 26397);
    }

    #[test]
    fn part2_provided_example() {
        let result = part2(&parse_input(
            r#"[({(<(())[]>[[{[]{<()<>>
[(()[<>])]({[<{<<[]>>(
{([(<{}[<>[]}>{[]{[(<()>
(((({<>}<{<{<>}{[]{[]{}
[[<[([]))<([[{}[[()]]]
[{[{({}]{}}([{[{{{}}([]
{<[[]]>}<{[{[{[]{()[[[]
[<(<(<(<{}))><([]([]()
<{([([[(<>()){}]>(<<{{
<{([{{}}[<[[[<>{}]]]>[]]"#,
        ));

        assert_eq!(result, 288957)
    }
}
