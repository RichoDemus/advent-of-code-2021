use std::collections::HashMap;

#[aoc_generator(day6)]
fn parse_input(input: &str) -> Vec<i32> {
    input
        .lines()
        .next()
        .unwrap()
        .split(',')
        .map(|str| str.parse().unwrap())
        .collect()
}

#[aoc(day6, part1)]
fn part1(fishies: &[i32]) -> usize {
    let mut fishies = fishies.to_vec();
    for _day in 0..80 {
        let mut new_fishies = vec![];
        for fish in &mut fishies {
            if fish == &0 {
                *fish = 6;
                new_fishies.push(8);
            } else {
                *fish -= 1;
            }
        }
        fishies.append(&mut new_fishies);
    }
    fishies.len()
}

#[aoc(day6, part1, map)]
fn part1_map(fishies: &[i32]) -> u128 {
    let mut fish_map: HashMap<i32, u128> = HashMap::new();
    for fish in fishies {
        *fish_map.entry(*fish).or_insert(0) += 1;
    }

    for _day in 1..=80 {
        let mut new_fishies = 0;
        let mut parents = 0;
        for fish_amount in 0..=8 {
            if fish_amount == 0 {
                let amount = fish_map.get(&fish_amount).unwrap_or(&0);
                new_fishies = *amount;
                parents = *amount;
                fish_map.insert(0, 0);
            } else {
                let amount = *fish_map.get(&fish_amount).unwrap_or(&0);
                fish_map.insert(fish_amount - 1, amount);
            }
        }
        *fish_map.entry(6).or_insert(0) += parents;
        fish_map.insert(8, new_fishies);
    }
    fish_map.values().fold(0, |acc, v| acc + *v)
}

#[aoc(day6, part2)]
fn part2(fishies: &[i32]) -> u128 {
    let mut fish_map: HashMap<i32, u128> = HashMap::new();
    for fish in fishies {
        *fish_map.entry(*fish).or_insert(0) += 1;
    }

    for _day in 1..=256 {
        let mut new_fishies = 0;
        let mut parents = 0;
        for fish_amount in 0..=8 {
            if fish_amount == 0 {
                let amount = fish_map.get(&fish_amount).unwrap_or(&0);
                new_fishies = *amount;
                parents = *amount;
                fish_map.insert(0, 0);
            } else {
                let amount = *fish_map.get(&fish_amount).unwrap_or(&0);
                fish_map.insert(fish_amount - 1, amount);
            }
        }
        *fish_map.entry(6).or_insert(0) += parents;
        fish_map.insert(8, new_fishies);
    }
    fish_map.values().fold(0, |acc, v| acc + *v)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn verify_part1() {
        let input = include_str!("../input/2021/day6.txt");
        let input = parse_input(input);
        assert_eq!(part1(input.as_slice()), 353079);
        assert_eq!(part1_map(input.as_slice()), 353079);
    }

    #[test]
    fn verify_part2() {
        let input = include_str!("../input/2021/day6.txt");
        assert_eq!(part2(parse_input(input).as_slice()), 1605400130036);
    }

    #[test]
    fn part1_provided_example() {
        let result = part1(&parse_input(r#"3,4,3,1,2"#));

        assert_eq!(result, 5934);
    }

    #[test]
    fn part2_provided_example() {
        let result = part2(&parse_input(r#"3,4,3,1,2"#));

        assert_eq!(result, 26984457539)
    }
}
