#[aoc_generator(day7)]
fn parse_input(input: &str) -> Vec<i64> {
    input
        .lines().next().unwrap()
        .split(',')
        .map(|s| s.parse().expect("parse int"))
        .collect::<Vec<_>>()
}


#[aoc(day7, part1)]
fn part1(input: &[i64]) -> i64 {
    let min = *input.iter().min().unwrap();
    let max = *input.iter().max().unwrap();

    let mut lowest_fuel_cost = (0, None);
    for position in min..=max {
        let mut fuel_cost = 0;
        for crabmarine in input {
            fuel_cost += (crabmarine - position).abs();
        }
        lowest_fuel_cost = match lowest_fuel_cost.1 {
            None => (position, Some(fuel_cost)),
            Some(current_fuel_cost) if current_fuel_cost > fuel_cost => (position, Some(fuel_cost)),
            _ => lowest_fuel_cost,
        };
    }
    return lowest_fuel_cost.1.unwrap()
}

#[aoc(day7, part2)]
fn part2(input: &[i64]) -> i64 {
    let min = *input.iter().min().unwrap();
    let max = *input.iter().max().unwrap();
    let mut lowest_fuel_cost = 0;
    for position in min..=max {
        let mut fuel_cost:i64 = 0;
        for crabmarine in input {
            let mut cost = 0;
            let mut incr = 1;
            let steps = (crabmarine - position).abs();
            for _ in 0..steps {
                cost += incr;
                incr += 1;
            }
            fuel_cost += cost;
        }
        if lowest_fuel_cost == 0 {
            lowest_fuel_cost = fuel_cost;
        }
        lowest_fuel_cost = lowest_fuel_cost.min(fuel_cost);
    }
    lowest_fuel_cost
}

#[cfg(test)]
mod tests {
    use super::*;

#[test]
    fn verify_part1() {
        let input = include_str!("../input/2021/day7.txt");
        let input = parse_input(input);
        assert_eq!(part1(input.as_slice()), 328187);
    }

    // #[test]
    // fn verify_part2() {
    //     let input = include_str!("../input/2021/day7.txt");
    //     assert_eq!(part2(parse_input(input).as_slice()), 19081);
    // }

    #[test]
    fn part1_provided_example() {
        let result = part1(&parse_input(
            r#"16,1,2,0,4,2,7,1,2,14"#,
        ));

        assert_eq!(result, 37);
    }

    #[test]
    fn part2_provided_example() {
        let result = part2(&parse_input(
            r#"16,1,2,0,4,2,7,1,2,14"#,
        ));

        assert_eq!(result, 168);
    }
}
