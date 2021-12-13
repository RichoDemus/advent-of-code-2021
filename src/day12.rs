use std::collections::{HashMap, HashSet};

#[aoc_generator(day12)]
fn parse_input(input: &str) -> Vec<(String, String)> {
    input.lines().map(|line|{
        let mut caves = line.split('-');
        let left = caves.next().unwrap();
        let right = caves.next().unwrap();
        (left.to_string(), right.to_string())
    }).collect()
}


#[aoc(day12, part1)]
fn part1(input: &[(String, String)]) -> i64 {
    let mut paths = HashMap::new();
    for (left, right) in input {
        paths.entry(left.clone()).or_insert_with(HashSet::new).insert(right.to_string());
        paths.entry(right.clone()).or_insert_with(HashSet::new).insert(left.to_string());
    }

    calculate_paths_rec(&paths, HashSet::new(), vec!["start".to_string()].as_slice())
}

fn calculate_paths_rec(paths: &HashMap<String, HashSet<String>>, mut already_visited_nodes: HashSet<String>, current_path: &[String]) -> i64 {
    let current_cave = current_path.last().unwrap();
    if &current_cave.to_lowercase() == current_cave {
    already_visited_nodes.insert(current_cave.clone());
    }
        if current_cave == "end" {
            return 1;
        }

    // we are not at the end
    let caves_we_can_go_to = paths.get(current_cave).unwrap();
    let mut paths_from_here = 0;
    for destination in caves_we_can_go_to {
        if already_visited_nodes.contains(destination) {
            continue
        }
        let mut new_current_path = current_path.to_owned();
        new_current_path.push(destination.clone());
        paths_from_here += calculate_paths_rec(paths, already_visited_nodes.clone(), new_current_path.as_slice());
    }

    paths_from_here
}

#[aoc(day12, part2)]
fn part2(input: &[(String, String)]) -> i64 {
    let mut paths = HashMap::new();
    for (left, right) in input {
        paths.entry(left.clone()).or_insert_with(HashSet::new).insert(right.to_string());
        paths.entry(right.clone()).or_insert_with(HashSet::new).insert(left.to_string());
    }

    calculate_paths_rec2(&paths, HashSet::new(), vec!["start".to_string()].as_slice(), 0)
}

fn calculate_paths_rec2(paths: &HashMap<String, HashSet<String>>, mut already_visited_nodes: HashSet<String>, current_path: &[String], number_of_single_cave_overrides: u8) -> i64 {
    if number_of_single_cave_overrides > 1 {
        return 0;
    }
    let current_cave = current_path.last().unwrap();
    if &current_cave.to_lowercase() == current_cave {
        already_visited_nodes.insert(current_cave.clone());
    }
    if current_cave == "end" {
        return 1;
    }

    // we are not at the end
    let caves_we_can_go_to = paths.get(current_cave).unwrap();
    let mut paths_from_here = 0;
    for destination in caves_we_can_go_to {
        if destination == &"start".to_string() {
            continue
        }
        let number_of_single_cave_overrides = if already_visited_nodes.contains(destination) {
            number_of_single_cave_overrides + 1
        } else {
            number_of_single_cave_overrides
        };

        let mut new_current_path = current_path.to_owned();
        new_current_path.push(destination.clone());
        paths_from_here += calculate_paths_rec2(paths, already_visited_nodes.clone(), new_current_path.as_slice(), number_of_single_cave_overrides);
    }

    paths_from_here
}

#[cfg(test)]
mod tests {
    use super::*;

#[test]
    fn verify_part1() {
        let input = include_str!("../input/2021/day12.txt");
        let input = parse_input(input);
        assert_eq!(part1(input.as_slice()), 3298);
    }

    #[test]
    fn verify_part2() {
        let input = include_str!("../input/2021/day12.txt");
        assert_eq!(part2(parse_input(input).as_slice()), 93572);
    }

    #[test]
    fn part1_provided_example() {
        let result = part1(&parse_input(
            r#"start-A
start-b
A-c
A-b
b-d
A-end
b-end"#,
        ));

        assert_eq!(result, 10)
    }

    #[test]
    fn part2_provided_example() {
        let result = part2(&parse_input(
            r#"start-A
start-b
A-c
A-b
b-d
A-end
b-end"#,
        ));

        assert_eq!(result, 36)
    }
}
