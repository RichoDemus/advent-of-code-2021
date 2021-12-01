#[aoc(day1, part1)]
fn part1_error_scanning_rate(input: &str) -> u64 {
    let mut last_number = None;
    let mut increases = 0;
    for measure in input.lines() {
        let number:i32 = measure.parse().unwrap();
        if last_number.is_none() {
            last_number = Some(number);
            continue
        }
        let prev = last_number.unwrap();
        if number > prev {
            increases += 1;
        }
        last_number = Some(number);
    }

    increases
}

#[aoc(day1, part2)]
fn part2_sliding_windows(input: &str) -> u64 {
    let input = input.lines().map(|s|s.parse::<i32>().unwrap()).collect::<Vec<_>>();

    let mut last_number = None;
    let mut increases = 0;
    for i in 0..input.len()-2 {
        let new_sum = input.get(i).unwrap() +input.get(i+1).unwrap()+input.get(i+2).unwrap();
        if let Some(last_number) = last_number {
            if new_sum > last_number {
                increases += 1;
            }
        }
        last_number = Some(new_sum);
    }
    increases
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn verify_part1() {
        let input = include_str!("../input/2021/day1.txt");
        assert_eq!(part1_error_scanning_rate(input), 1688);
    }

    #[test]
    fn verify_part2() {
        let input = include_str!("../input/2021/day1.txt");
        assert_eq!(part2_sliding_windows(input), 1728);
    }

    #[test]
    fn part1_provided_example() {
        let result = part1_error_scanning_rate(r#"199
200
208
210
200
207
240
269
260
263"#);

        assert_eq!(7, result);
    }

    #[test]
    fn part2_provided_example() {
        let result = part2_sliding_windows(r#"199
200
208
210
200
207
240
269
260
263"#);

        assert_eq!(result, 5);
    }

}
