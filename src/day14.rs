use std::collections::HashMap;
use std::sync::Mutex;

#[aoc_generator(day14)]
fn parse_input(input: &str) -> Input {
    let mut split = input.split("\n\n");
    let template = split.next().unwrap().chars().collect();
    let rules = split
        .next()
        .unwrap()
        .lines()
        .map(|line| {
            let mut split = line.split("->");
            let mut pair = split.next().unwrap().trim().chars();
            let left = pair.next().unwrap();
            let right = pair.next().unwrap();
            let output = split.next().unwrap().trim().chars().next().unwrap();
            Rule {
                left,
                right,
                output,
            }
        })
        .collect();
    Input { template, rules }
}

#[derive(Debug)]
struct Input {
    template: Vec<char>,
    rules: Vec<Rule>,
}

impl Input {
    fn convert(&self, element_one: char, element_two: char) -> char {
        for rule in &self.rules {
            if rule.left != element_one {
                continue;
            }
            if rule.right != element_two {
                continue;
            }
            return rule.output;
        }
        panic!("No rule")
    }
}

fn calc_new_element(rules: &[Rule], element_one: char, element_two: char) -> char {
    for rule in rules {
        if rule.left != element_one {
            continue;
        }
        if rule.right != element_two {
            continue;
        }
        return rule.output;
    }
    panic!("No rule")
}

#[derive(Debug)]
struct Rule {
    left: char,
    right: char,
    output: char,
}

#[aoc(day14, part1)]
fn part1(input: &Input) -> u64 {
    let mut result = input.template.clone();
    for _ in 1..=10 {
        let mut temp = vec![result[0]];
        for pair in result.windows(2) {
            let left = pair[0];
            let right = pair[1];
            let middle = input.convert(left, right);
            temp.push(middle);
            temp.push(right);
        }
        result = temp;
    }

    let quantities: HashMap<char, u64> =
        result.into_iter().fold(HashMap::new(), |mut acc, next| {
            *acc.entry(next).or_insert(0) += 1;
            acc
        });

    let (_c, most_common) = quantities
        .iter()
        .max_by_key(|(_char, quantity)| **quantity)
        .unwrap();
    let (_c2, least_common) = quantities
        .iter()
        .min_by_key(|(_char, quantity)| **quantity)
        .unwrap();

    *most_common - *least_common
}

#[aoc(day14, part2)]
fn part2(input: &Input) -> u64 {
    let lookup_table: HashMap<(char, char, i32), HashMap<char, u64>> = HashMap::new();

    let lookup_table = Mutex::new(lookup_table);

    let occurances = input
        .template
        .clone()
        .windows(2)
        .map(|pair| {
            let left = pair[0];
            let right = pair[1];
            let mut occurances = calculate_occurances(&input.rules, left, right, 1, &lookup_table);
            *occurances.entry(left).or_insert(0) += 1;
            *occurances.entry(right).or_insert(0) += 1;
            occurances
        })
        .fold(HashMap::new(), |mut acc, next| {
            for (k, v) in next {
                *acc.entry(k).or_insert(0) += v;
            }
            acc
        });

    let (_mc_char, most_common) = occurances
        .iter()
        .max_by_key(|(_char, quantity)| **quantity)
        .unwrap();
    let (_lc_char, least_common) = occurances
        .iter()
        .min_by_key(|(_char, quantity)| **quantity)
        .unwrap();

    *most_common - *least_common
}

type LookupTable = Mutex<HashMap<(char, char, i32), HashMap<char, u64>>>;

fn calculate_occurances(
    rules: &[Rule],
    left: char,
    right: char,
    step: i32,
    lookup_table: &LookupTable,
) -> HashMap<char, u64> {
    if step >= 40 {
        let mut result = HashMap::new();
        let new = calc_new_element(rules, left, right);
        *result.entry(new).or_insert(0) += 1;
        return result;
    }

    match lookup_table.lock().expect("lock").get(&(left, right, step)) {
        None => {
            // we haven't already fixed this occurance, do some code
            // but lets do it below
        }
        Some(result) => {
            // we have already calculated this
            return result.clone();
        }
    }

    let middle = calc_new_element(rules, left, right);
    let mut first_pair_occurances =
        calculate_occurances(rules, left, middle, step + 1, lookup_table);
    {
        let mut table = lookup_table.lock().expect("lock");
        table.insert((left, middle, step + 1), first_pair_occurances.clone());
    }
    let second_pair_occurances = calculate_occurances(rules, middle, right, step + 1, lookup_table);
    {
        let mut table = lookup_table.lock().expect("lock");
        table.insert((middle, right, step + 1), second_pair_occurances.clone());
    }
    // merge hashmaps
    for (k, v) in second_pair_occurances {
        *first_pair_occurances.entry(k).or_insert(0) += v;
    }

    *first_pair_occurances.entry(middle).or_insert(0) += 1;
    first_pair_occurances
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn verify_part1() {
        let input = include_str!("../input/2021/day14.txt");
        let input = parse_input(input);
        assert_eq!(part1(&input), 3342);
    }

    #[test]
    fn verify_part2() {
        let input = include_str!("../input/2021/day14.txt");
        assert_eq!(part2(&parse_input(input)), 3776553567525);
    }

    #[test]
    fn part1_provided_example() {
        let result = part1(&parse_input(
            r#"NNCB

CH -> B
HH -> N
CB -> H
NH -> C
HB -> C
HC -> B
HN -> C
NN -> C
BH -> H
NC -> B
NB -> B
BN -> B
BB -> N
BC -> B
CC -> N
CN -> C"#,
        ));

        assert_eq!(result, 1588)
    }

    #[test]
    fn part2_provided_example() {
        let result = part2(&parse_input(
            r#"NNCB

CH -> B
HH -> N
CB -> H
NH -> C
HB -> C
HC -> B
HN -> C
NN -> C
BH -> H
NC -> B
NB -> B
BN -> B
BB -> N
BC -> B
CC -> N
CN -> C"#,
        ));

        assert_eq!(result, 2188189693529)
    }
}
