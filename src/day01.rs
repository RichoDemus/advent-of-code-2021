#[aoc_generator(day1)]
fn parse_input(input: &str) -> Vec<u64> {
    input.lines().map(|line| line.parse().unwrap()).collect()
}

#[aoc(day1, part1)]
fn part1_error_scanning_rate(input: &[u64]) -> u64 {
    let mut previous_number = input[0];
    let mut increases = 0;
    for number in input.iter().skip(1) {
        if *number > previous_number {
            increases += 1;
        }
        previous_number = *number;
    }
    increases
}

#[aoc(day1, part1, fold)]
fn part1_error_scanning_rate_fold(input: &[u64]) -> u64 {
    let (increases, _) =
        input.iter().fold(
            (0, None),
            |(increases, previous_number), new| match previous_number {
                Some(previous_number) if new > previous_number => (increases + 1, Some(new)),
                // None => (increases, Some(new)),
                // Some(_) => (increases, Some(new)),
                _ => (increases, Some(new)),
            },
        );
    increases
}

#[aoc(day1, part1, windows)]
fn part1_error_scanning_rate_windows(input: &[u64]) -> usize {
    input
        .windows(2)
        .filter(|window| window[1] > window[0])
        .count()
}

#[aoc(day1, part2)]
fn part2_sliding_windows(input: &[u64]) -> u64 {
    let mut last_number = None;
    let mut increases = 0;
    for i in 0..input.len() - 2 {
        let new_sum = input[i] + input[i + 1] + input[i + 2];
        if let Some(last_number) = last_number {
            if new_sum > last_number {
                increases += 1;
            }
        }
        last_number = Some(new_sum);
    }
    increases
}

#[aoc(day1, part2, trick)]
fn part_2_trick(input: &[u64]) -> u64 {
    input.windows(4).filter(|v| v[3] > v[0]).count() as u64
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn verify_part1() {
        let input = include_str!("../input/2021/day1.txt");
        let input = parse_input(input);
        assert_eq!(part1_error_scanning_rate(input.as_slice()), 1688);
        assert_eq!(part1_error_scanning_rate_fold(input.as_slice()), 1688);
        assert_eq!(part1_error_scanning_rate_windows(input.as_slice()), 1688);
    }

    #[test]
    fn verify_part2() {
        let input = include_str!("../input/2021/day1.txt");
        assert_eq!(part2_sliding_windows(parse_input(input).as_slice()), 1728);
        assert_eq!(part_2_trick(parse_input(input).as_slice()), 1728);
    }

    #[test]
    fn part1_provided_example() {
        let result = part1_error_scanning_rate(&[199, 200, 208, 210, 200, 207, 240, 269, 260, 263]);

        assert_eq!(7, result);
    }

    #[test]
    fn part2_provided_example() {
        let result = part2_sliding_windows(&[199, 200, 208, 210, 200, 207, 240, 269, 260, 263]);

        assert_eq!(result, 5);
    }
}
