use std::collections::HashMap;
use std::ops::Not;

use itertools::Itertools;
use split_iter::Splittable;

#[aoc_generator(day8)]
fn parse_input(input: &str) -> Vec<Line> {
    input
        .lines()
        .filter_map(|line| {
            if line.is_empty() {
                return None;
            }
            let mut split = line.split('|');
            let signals = split
                .next()
                .unwrap_or_else(|| panic!("can't parse: {:?}", line))
                .trim();
            let output = split
                .next()
                .unwrap_or_else(|| panic!("can't parse: {:?}", line))
                .trim();

            let signals = signals
                .split_whitespace()
                .map(std::string::ToString::to_string)
                .map(|segments| Segments { segments })
                .collect::<Vec<_>>();
            let output = output
                .split_whitespace()
                .map(std::string::ToString::to_string)
                .map(|segments| Segments { segments })
                .collect::<Vec<_>>();
            Some(Line { signals, output })
        })
        .collect()
}

#[derive(Debug, Clone)]
struct Line {
    signals: Vec<Segments>,
    output: Vec<Segments>,
}

#[derive(Debug, Clone)]
struct Segments {
    segments: String,
}

impl Segments {
    fn possible_digits(&self) -> Vec<i64> {
        let possible_digits: Vec<i64> = match self.segments.len() {
            2 => vec![1],
            3 => vec![7],
            4 => vec![4],
            5 => vec![2, 3, 5],
            6 => vec![0, 6, 9],
            7 => vec![8],
            _ => panic!("No idea what this digit is"),
        };
        possible_digits
        // println!("\tInitial possible digits: {:?}", possible_digits);
        // possible_digits.into_iter()
        //     .filter(|digit|{
        //         let actual_segments = digit_to_segments(*digit);
        //         for false_segment in self.segments.chars(){
        //             let possible_segments = mappings.get(&false_segment).unwrap();
        //             for s in possible_segments {
        //                 if actual_segments.contains(s).not() {
        //                     println!("\t\t{} not in {:?}, removing from result", s, actual_segments);
        //                     return false;
        //                 }
        //             }
        //         }
        //         true
        //     }).collect()
    }
}

#[derive(Debug)]
struct Mappings {
    mappings: HashMap<char, Vec<char>>,
}

impl Default for Mappings {
    fn default() -> Self {
        let mut mappings = HashMap::new();
        for segment in ['a', 'b', 'c', 'd', 'e', 'f', 'g'] {
            mappings.insert(segment, vec!['a', 'b', 'c', 'd', 'e', 'f', 'g']);
        }
        Self { mappings }
    }
}

// impl Mappings {
//     fn update_mappings(&mut self, segments: &Segments) {
//         let possible_digits = segments.possible_digits(&self.mappings);
//         println!("Checking mappings for {:?} {}. Possible digits: {:?}", segments.segments, segments.segments.len(), possible_digits);
//         if possible_digits.len() == 1 {
//             let digit = possible_digits[0];
//             let possible_segments = digit_to_segments(digit);
//             // for actual_segment in digit_segments {
//                 for false_segment in segments.segments.chars() {
//                     let asd = self.mappings.get_mut(&false_segment).unwrap();
//                     asd.retain(|c| possible_segments.contains(c));
//                 }
//             // }
//             println!("\tSegments {:?} can only be {} ({:?}). Mappings are now: {:?}", segments.segments, digit, possible_segments, self.mappings);
//         }
//     }
// }

#[allow(dead_code)]
fn digit_to_segments2(digit: i64) -> Vec<char> {
    match digit {
        0 => vec!['a', 'b', 'c', 'e', 'f', 'g'],
        1 => vec!['c', 'f'],
        2 => vec!['a', 'c', 'd', 'e', 'g'],
        3 => vec!['a', 'c', 'd', 'f', 'g'],
        4 => vec!['b', 'c', 'd', 'f'],
        5 => vec!['a', 'b', 'd', 'f', 'g'],
        6 => vec!['a', 'b', 'd', 'e', 'f', 'g'],
        7 => vec!['a', 'c', 'f'],
        8 => vec!['a', 'b', 'c', 'd', 'e', 'f', 'g'],
        9 => vec!['a', 'b', 'c', 'd', 'f', 'g'],
        _ => panic!("Bad digit: {}", digit),
    }
}

// fn add_mappings(mappings: &mut HashMap<char, Vec<char>>, segments: &str, digit: i64) {
//     for char in segments.chars() {
//         match digit {
//             1 => mappings.entry(char).insert(vec![]).get_mut().append(&mut vec!['c','f']),
//             _ => panic!("add mappings not implemented for {}", digit),
//         }
//     }
//
// }

// #[aoc(day8, part1)]
// fn part1(input: &[Line]) -> usize {
//     // let has_one = input.iter().all(|line|line.signals.iter().any(|segments|segments.segments.len() == 2));
//     // let has_seven = input.iter().all(|line|line.signals.iter().any(|segments|segments.segments.len() == 3));
//     // let has_four = input.iter().all(|line|line.signals.iter().any(|segments|segments.segments.len() == 4));
//     // let has_235 = input.iter().all(|line|line.signals.iter().any(|segments|segments.segments.len() == 5));
//     // let has_069 = input.iter().all(|line|line.signals.iter().any(|segments|segments.segments.len() == 6));
//     // let has_eight = input.iter().all(|line|line.signals.iter().any(|segments|segments.segments.len() == 7));
//     // panic!("Does all segments have a 1?: {}, 7?{} 4?{} 8?{}. 235?{} 069?{}", has_one, has_seven, has_four, has_eight, has_235, has_069);
//     // println!("input: {:?}", input);
//     let digits = vec![1,2,3,4,5,6,7,8,9,0];
//     for line in input {
//         let mut mappings = Mappings::default();
//         // println!("Mappings: {:?}", mappings);
//         for _ in 0..10 {
//             for segments in &line.signals {
//                 mappings.update_mappings(segments);
//
//                 // println!("Mappings: {:?}", mappings);
//
//
//             }
//
//         }
//
//     }
//     todo!()
// }
#[allow(dead_code)]
fn is_in(outer: &str, inner: &str) -> bool {
    let result = inner
        .chars()
        .into_iter()
        .all(|segment| outer.chars().contains(&segment));
    // println!("\tIs {:?} in {:?}? {}", inner, outer, result);
    result
}

#[aoc(day8, part2)]
#[allow(clippy::too_many_lines)]
fn part2(input: &[Line]) -> usize {
    let mut total_sum = 0;
    for line in input {
        let mut known = HashMap::new();
        let mut segments = line.signals.clone();
        segments.retain(|segment| {
            let digits = segment.possible_digits();
            if digits.len() == 1 {
                let digit = digits[0];
                known.insert(digit, segment.segments.clone());
                return false;
            }
            true
        });
        println!("took the easy ones: {:?}, left are {:?}", known, segments);

        let (could_be_235, could_be_069) = segments.into_iter().split(|s| s.segments.len() == 6);
        let could_be_235 = could_be_235.collect::<Vec<_>>();
        let could_be_069 = could_be_069.collect::<Vec<_>>();

        println!("235: {:?}", could_be_235);
        println!("069: {:?}", could_be_069);

        let one = known.get(&1).unwrap().clone();
        let four = known.get(&4).unwrap().clone();
        println!("1: {:?}", one);
        println!("4: {:?}", four);

        // 3 is the only one from 235 with a 1 in it
        // "4-1" is in 5 but not in 2

        // 6 is the only one without 1 in it
        // "4-1" is in 9 but not in 0

        // lets make the magical 4-1 thing
        let four_minues_one = {
            let mut fours = four.chars().collect::<Vec<char>>();
            let ones = one.chars().collect::<Vec<char>>();
            fours.retain(|c| ones.contains(c).not());
            fours
        };

        //find 3
        let (two_and_five, three) = could_be_235.into_iter().split(|s| {
            one.chars()
                .all(|one_char| s.segments.chars().contains(&one_char))
        });
        let three = three.collect::<Vec<_>>();
        let two_and_five = two_and_five.collect::<Vec<_>>();

        println!("2&5: {:?}", two_and_five);

        // find 5
        let (two, five) = two_and_five.into_iter().split(|s| {
            four_minues_one
                .iter()
                .all(|char| s.segments.chars().contains(char))
        });
        let five = five.collect::<Vec<_>>();
        let two = two.collect::<Vec<_>>();

        //find 6
        let (six, zero_and_nine) = could_be_069.into_iter().split(|s| {
            one.chars()
                .all(|one_char| s.segments.chars().contains(&one_char))
        });
        let six = six.collect::<Vec<_>>();
        let zero_and_nine = zero_and_nine.collect::<Vec<_>>();

        println!("0&9: {:?}", zero_and_nine);

        // find 9
        let (zero, nine) = zero_and_nine.into_iter().split(|s| {
            four_minues_one
                .iter()
                .all(|char| s.segments.chars().contains(char))
        });
        let nine = nine.collect::<Vec<_>>();
        let zero = zero.collect::<Vec<_>>();
        let zero = zero[0].segments.clone();
        let two = two[0].segments.clone();
        let three = three[0].segments.clone();
        let five = five[0].segments.clone();
        let six = six[0].segments.clone();
        let nine = nine[0].segments.clone();
        let seven = known.get(&7).unwrap().clone();
        let eight = known.get(&8).unwrap().clone();
        println!("0: {:?}", zero);
        println!("1: {:?}", one);
        println!("2: {:?}", two);
        println!("3: {:?}", three);
        println!("4: {:?}", four);
        println!("5: {:?}", five);
        println!("6: {:?}", six);
        println!("7: {:?}", seven);
        println!("8: {:?}", eight);
        println!("9: {:?}", nine);

        let mut damn_digits = HashMap::new();
        damn_digits.insert(zero.chars().sorted().collect::<Vec<_>>(), 0);
        damn_digits.insert(one.chars().sorted().collect::<Vec<_>>(), 1);
        damn_digits.insert(two.chars().sorted().collect::<Vec<_>>(), 2);
        damn_digits.insert(three.chars().sorted().collect::<Vec<_>>(), 3);
        damn_digits.insert(four.chars().sorted().collect::<Vec<_>>(), 4);
        damn_digits.insert(five.chars().sorted().collect::<Vec<_>>(), 5);
        damn_digits.insert(six.chars().sorted().collect::<Vec<_>>(), 6);
        damn_digits.insert(seven.chars().sorted().collect::<Vec<_>>(), 7);
        damn_digits.insert(eight.chars().sorted().collect::<Vec<_>>(), 8);
        damn_digits.insert(nine.chars().sorted().collect::<Vec<_>>(), 9);

        let numbers = line
            .output
            .iter()
            .map(|output_segments| {
                output_segments
                    .segments
                    .chars()
                    .sorted()
                    .collect::<Vec<_>>()
            })
            .map(|output_sequence| {
                let asd = *damn_digits.get(&output_sequence).expect("PLS BE HERE");
                asd
            })
            .collect::<Vec<_>>();

        let sum = numbers[0] * 1000 + numbers[1] * 100 + numbers[2] * 10 + numbers[3];
        println!("sum: {}", sum);

        total_sum += sum;
    }

    total_sum
}

// #[aoc(day8, part2)]
#[allow(dead_code, clippy::too_many_lines)]
fn part2_old(input: &[Line]) -> usize {
    for line in input {
        let digits = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 0];
        let mut valid_permutations = 0;
        'perm: for permutation in digits.iter().permutations(10) {
            for (i, segments) in line.signals.iter().enumerate() {
                let desired_digit = permutation[i];
                if segments.possible_digits().contains(desired_digit).not() {
                    continue 'perm;
                }
            }
            // maybe found something
            println!("Possible numbers: {:?}", permutation);
            println!("Segments: {:?}", line.signals);
            // lets do some checks
            let _potential_zero = line
                .signals
                .get(permutation.iter().position(|digit| **digit == 0).unwrap())
                .unwrap()
                .segments
                .as_str();
            let _potential_one = line
                .signals
                .get(permutation.iter().position(|digit| **digit == 1).unwrap())
                .unwrap()
                .segments
                .as_str();
            let _potential_two = line
                .signals
                .get(permutation.iter().position(|digit| **digit == 2).unwrap())
                .unwrap()
                .segments
                .as_str();
            let _potential_three = line
                .signals
                .get(permutation.iter().position(|digit| **digit == 3).unwrap())
                .unwrap()
                .segments
                .as_str();
            let _potential_four = line
                .signals
                .get(permutation.iter().position(|digit| **digit == 4).unwrap())
                .unwrap()
                .segments
                .as_str();
            let _potential_five = line
                .signals
                .get(permutation.iter().position(|digit| **digit == 5).unwrap())
                .unwrap()
                .segments
                .as_str();
            let _potential_six = line
                .signals
                .get(permutation.iter().position(|digit| **digit == 6).unwrap())
                .unwrap()
                .segments
                .as_str();
            let _potential_seven = line
                .signals
                .get(permutation.iter().position(|digit| **digit == 7).unwrap())
                .unwrap()
                .segments
                .as_str();
            let _potential_eight = line
                .signals
                .get(permutation.iter().position(|digit| **digit == 8).unwrap())
                .unwrap()
                .segments
                .as_str();
            let _potential_nine = line
                .signals
                .get(permutation.iter().position(|digit| **digit == 9).unwrap())
                .unwrap()
                .segments
                .as_str();

            // format left is in right
            let contain_rules = vec![
                (1, 0),
                (5, 6),
                (7, 0),
                (7, 3),
                (7, 8),
                (3, 9),
                (4, 8),
                (4, 9),
            ];

            for (inner, outer) in contain_rules {
                let inner2 = line
                    .signals
                    .get(
                        permutation
                            .iter()
                            .position(|digit| **digit == inner)
                            .unwrap(),
                    )
                    .unwrap()
                    .segments
                    .as_str();
                let outer2 = line
                    .signals
                    .get(
                        permutation
                            .iter()
                            .position(|digit| **digit == outer)
                            .unwrap(),
                    )
                    .unwrap()
                    .segments
                    .as_str();
                if is_in(outer2, inner2).not() {
                    println!("\t{} in {} failed", inner, outer);
                    continue 'perm;
                }
            }

            // if is_in(potential_nine, potential_three).not() {
            //     continue 'perm;
            // }
            //
            // if is_in(potential_nine, potential_seven).not() && is_in(potential_eight, potential_seven).not() && is_in(potential_three, potential_seven).not() {
            //     continue 'perm;
            // }
            //
            // if is_in(potential_zero, potential_one) {
            //     continue 'perm;
            // }
            //
            // if is_in(potential_six, potential_five) {
            //     continue 'perm;
            // }

            // panic!("Found one permutation: {:?}", permutation);
            // we have a solution!
            valid_permutations += 1;
        }
        if valid_permutations != 1 {
            panic!("Got {} permutations", valid_permutations);
        }
    }

    todo!()
}

#[aoc(day8, part1)]
fn part1(lines: &[Line]) -> usize {
    lines
        .iter()
        .map(|line| {
            line.output
                .iter()
                .filter(|segments| segments.possible_digits().len() == 1)
                .count()
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn verify_part1() {
        let input = include_str!("../input/2021/day8.txt");
        let input = parse_input(input);
        assert_eq!(part1(input.as_slice()), 301);
    }

    #[test]
    fn verify_part2() {
        let input = include_str!("../input/2021/day8.txt");
        assert_eq!(part2(parse_input(input).as_slice()), 908067);
    }

    #[test]
    fn part1_provided_example() {
        let result = part1(&parse_input(
            r#"be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe
edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc
fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg
fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb
aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea
fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb
dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe
bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef
egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb
gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce"#,
        ));

        assert_eq!(result, 26)
    }

    #[test]
    fn part2_provided_example() {
        let result = part2(&parse_input(
            r#"be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe
edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc
fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg
fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb
aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea
fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb
dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe
bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef
egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb
gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce"#,
        ));

        assert_eq!(result, 61229)
    }

    #[test]
    fn part2_mini_example() {
        let result = part2(&parse_input(
            r#"acedgfb cdfbe gcdfa fbcad dab cefabd cdfgeb eafb cagedb ab | cdfeb fcadb cdfeb cdbaf"#,
        ));

        assert_eq!(result, 5353)
    }
}
