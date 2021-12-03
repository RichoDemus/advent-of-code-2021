use split_iter::Splittable;

#[aoc_generator(day3)]
fn parse_input(input: &str) -> Vec<Vec<u8>> {
    fn parse_line(str: &str) -> Vec<u8> {
        #[allow(clippy::cast_possible_truncation)]
        str.chars().map(|c| c.to_digit(10).unwrap() as u8).collect()
    }

    input.lines().map(parse_line).collect()
}

#[aoc(day3, part1)]
fn part1(input: &[Vec<u8>]) -> u64 {
    let mut gamma_rate = "".to_string();
    let mut epsilon_rate = "".to_string();

    let mut column = 0;

    'outer: loop {
        let mut zeroes = 0;
        let mut ones = 0;
        for line in input {
            let bit = match line.get(column) {
                None => break 'outer,
                Some(b) => b,
            };
            match bit {
                0 => zeroes += 1,
                1 => ones += 1,
                _ => panic!("wat"),
            }
        }
        if zeroes > ones {
            gamma_rate += "0";
            epsilon_rate += "1";
        } else {
            gamma_rate += "1";
            epsilon_rate += "0";
        }
        column += 1;
        // panic!("zeroes: {}, ones: {}, gamma: {}, epsilon: {}", zeroes, ones, gamma_rate, epsilon_rate);
    }

    let gamma_rate2: u64 = u64::from_str_radix(gamma_rate.as_str(), 2).unwrap();
    let epsilon_rate2: u64 = u64::from_str_radix(epsilon_rate.as_str(), 2).unwrap();

    gamma_rate2 * epsilon_rate2
}

#[aoc(day3, part2)]
fn part2(input: &[Vec<u8>]) -> u64 {
    oxygen_generator_rating(input) * co2_scrubber_rating(input)
}

fn oxygen_generator_rating(report: &[Vec<u8>]) -> u64 {
    let mut report: Vec<Vec<u8>> = report.to_vec();
    for column in 0..12 {
        let (ones, zeroes) = report.into_iter().split(|i| i[column] == 0);
        let zeroes = zeroes.collect::<Vec<_>>();
        let ones = ones.collect::<Vec<_>>();

        report = if ones.len() < zeroes.len() {
            zeroes
        } else {
            ones
        };
        if report.len() == 1 {
            let stuff_str: String = report[0]
                .iter()
                .map(std::string::ToString::to_string)
                .collect::<String>();
            let result = u64::from_str_radix(stuff_str.as_str(), 2).unwrap();
            return result;
        }
    }
    panic!("Unreachable code")
}

fn co2_scrubber_rating(report: &[Vec<u8>]) -> u64 {
    let mut report: Vec<Vec<u8>> = report.to_vec();
    for column in 0..12 {
        let (ones, zeroes) = report.into_iter().split(|i| i[column] == 0);
        let zeroes = zeroes.collect::<Vec<_>>();
        let ones = ones.collect::<Vec<_>>();

        report = if ones.len() < zeroes.len() {
            ones
        } else {
            zeroes
        };
        if report.len() == 1 {
            let stuff_str: String = report[0]
                .iter()
                .map(std::string::ToString::to_string)
                .collect::<String>();
            let result = u64::from_str_radix(stuff_str.as_str(), 2).unwrap();
            return result;
        }
    }
    panic!("Unreachable code")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn verify_part1() {
        let input = include_str!("../input/2021/day3.txt");
        let input = parse_input(input);
        assert_eq!(part1(input.as_slice()), 2724524);
    }

    #[test]
    fn verify_part2() {
        let input = include_str!("../input/2021/day3.txt");
        assert_eq!(part2(parse_input(input).as_slice()), 2775870);
    }

    #[test]
    fn part1_provided_example() {
        let result = part1(&parse_input(
            r#"00100
11110
10110
10111
10101
01111
00111
11100
10000
11001
00010
01010"#,
        ));

        assert_eq!(result, 198)
    }

    #[test]
    fn part2_provided_example() {
        let input = parse_input(
            r#"00100
11110
10110
10111
10101
01111
00111
11100
10000
11001
00010
01010"#,
        );

        assert_eq!(
            oxygen_generator_rating(&input),
            23,
            "wrong oxygen generator rating"
        );
        assert_eq!(co2_scrubber_rating(&input), 10, "wrong co2 scrubber rating");

        let result = part2(&input);

        assert_eq!(result, 230, "wrong answer for part2")
    }
}
