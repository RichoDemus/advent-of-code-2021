use std::collections::VecDeque;
use std::fmt::{Display, Formatter};
use std::ops::Not;
use once_cell::unsync::Lazy;
use regex::Regex;
use crate::day18::ExplosionResult::NoExplosion;

// fn explode(number: &Number, depth: i64) -> ExplosionResult {
//     if depth == 4 {
//         // time to explode
//         return ExplosionResult::Exploded(number.clone());
//     }
//     // // println!("Exploding {}: {}", depth, number);
//     let (left_explosion_result, right_explosion_result ) = match number {
//         Number::Pair(left, right) => {
//             (explode(left, depth + 1),
//             explode(right, depth + 1))
//         }
//         Number::Literal(v) => return ExplosionResult::NoExplosion(self),
//     };
//
//     if let ExplosionResult::Exploded(exploded) = left_explosion_result {
//         // // println!("Left pair of {} exploded", number);
//
//     }
//
//     todo!()

#[aoc(day18, part1)]
fn part1(numbers: &str) -> i64 {
    let mut result = numbers.lines().next().unwrap().to_string();
    for (i, new) in numbers.lines().enumerate().skip(1) {
        result = proper_add(result.as_str(), new);
        // // println!("sum is now {}", new);
    }
    get_magnitude(result.as_str())
}

#[aoc(day18, part2)]
fn part2(numbers: &str) -> i64 {
    let mut largest_magnitude = 0;
    let numbers = numbers.lines().collect::<Vec<_>>();
    for i in 0..numbers.len() {
        for j in 0..numbers.len() {
            if i == j {
                continue
            }
            let add = proper_add(numbers.get(i).unwrap(), numbers.get(j).unwrap());
            let magnitude = get_magnitude(add.as_str());
            largest_magnitude = largest_magnitude.max(magnitude);
        }
    }

    largest_magnitude
}

fn simple_add(left: &str, right: &str) -> String {
    format!("[{},{}]", left, right)
}

fn simple_explode(number: &str) -> String {
    let mut brackets = 0;
    for (i,c) in number.chars().enumerate() {
        if c == '[' {
            brackets += 1;
        }
        if c == ']' {
            brackets -= 1;
        }
        if brackets == 5 {
            // we're at the start of a pair nested inside 4
            let exploding_pair_start = i;
            let exploding_pair_end = number[i..].find(']').unwrap() + i + 1;
            let pair = &number[exploding_pair_start..exploding_pair_end];
            let (left, right) = extract_numbers_from_pair(pair);
            // println!("\tExplosion at {} which is {}", i, pair);
            let mut new_str = format!("{}0{}", &number[0..exploding_pair_start], &number[exploding_pair_end..]);
            // // println!("there should be a zero here: {:?}", new_str.chars().nth(exploding_pair_start));
            for (i,c) in new_str.chars().enumerate().skip(exploding_pair_start + 1) {
                if c.is_numeric() {
                    // we found the digit to add, let's see how long it is
                    let mut end = i;
                    loop {
                        if new_str.chars().nth(end+1).unwrap().is_numeric() {
                            end += 1;
                        } else {
                            break;
                        }
                    }
                    let num = &new_str[i..=end];
                    let num:f32 = num.parse().unwrap_or_else(|_|panic!("Couldn't parse {}", num));
                    // println!("Inserting {} at {}-{}", num, i, end);
                    new_str = format!("{}{}{}", &new_str[0..i], right+num as i64, &new_str[end+1..]);
                    break;
                }
            }
            let mut i = exploding_pair_start -1;
            loop {
                let c = new_str.chars().nth(i).unwrap();
                if c.is_numeric() {
                    // we found the digit to add, let's see how long it is
                    let mut start = i;
                    loop {
                        if new_str.chars().nth(start -1).unwrap().is_numeric() {
                            start -= 1;
                        } else {
                            break;
                        }
                    }
                    // // println!("should add {} to {} at {}", left, c, i);
                    let num:f32 = new_str[start..=i].parse().unwrap();
                    // new_str = format!("{}{}{}", &new_str[0..start], left+num as i64, &new_str[i+2..]);
                    new_str = format!("{}{}{}", &new_str[0..start], left+num as i64, &new_str[i+1..]);
                    break;
                }
                if i > 0 {
                    i -=1;
                } else { break }
            }
            return new_str
        }
    }

    number.to_string()
}

fn extract_numbers_from_pair(pair:&str) -> (i64, i64) {
    let inside = &pair[1..pair.len()-1];
    let mut split = inside.split(',');
    (split.next().unwrap().parse().unwrap(), split.next().unwrap().parse().unwrap(), )
}

fn simple_split(number: &str) -> String {
    let mut result = number.to_string();
    for (i,c) in number.chars().enumerate() {
        if c.is_numeric().not() {
            continue;
        }
        // need to see how many of the next ones are numeric
        let mut end = i;
        loop {
            if number.chars().nth(end+1).unwrap().is_numeric() {
                end += 1;
            } else {
                break;
            }
        }
        if end-i ==0 {
            continue
        }
        let num:f32 = number[i..=end].parse().unwrap();
        if num > 9. {
            let new_pair = format!("[{},{}]", (num as f32 / 2.).floor(), (num as f32 / 2.).ceil());
            // println!("\t\t{} -> new pair: {}", num, new_pair);
            result = format!("{}{}{}", &result[0..i], new_pair, &result[end+1..]);
            return result;
        }
    }
    result
}

fn proper_add(left: &str, right: &str) -> String {
    let mut result = simple_add(left, right);
    // println!("Result after add: {}", result);
    reduce(result.as_str())
}

fn reduce(number: &str) -> String {
    // println!("Reducing {}", number);
    let mut failsafe = 1000;
    let mut result = number.to_string();
    loop {
        failsafe -= 1;
        if failsafe == 0 {
            panic!()
        }
        let new_number = simple_explode(result.as_str());
        if new_number != result {
            // println!("Exploded, is now: {}", new_number);
            result = new_number;
            // panic!("should be {}", "[[[[4,0],[5,4]],[[7,7],[6,0]]],[[8,14],[[0,15],[11,0]]]]");
            continue;
        }
        let new_number = simple_split(result.as_str());
        if new_number != result {
            // println!("Split, is now: {}", new_number);
            result = new_number;
            continue;
        }
        break;
    }
    return result;
}

fn get_magnitude(number:&str) -> i64 {
    if number.chars().all(char::is_numeric) {
        return number.parse().unwrap();
    }

    let (left, right) = split_pair(number);
    let left_value = get_magnitude(left);
    let right_value = get_magnitude(right);
    3*left_value  + 2*right_value
}

fn parse(number: &str) -> Number {
    if number.chars().all(char::is_numeric) {
        return Number::Literal(number.parse().unwrap());
    }
    let (left, right) = split_pair(number);
    let left = parse(left);
    let right = parse(right);
    Number::Pair(Box::new(left), Box::new(right))
}

fn split_pair(pair: &str) -> (&str, &str) {
    let mut opening_brackets = 0;
    for (i, c) in pair.chars().enumerate().skip(1){
        if c == '[' {
            opening_brackets += 1;
            continue
        }
        if c == ']' {
            opening_brackets -= 1;
            continue
        }
        if c == ',' && opening_brackets == 0 {
            // found the middle comma
            let (left, right) = pair.split_at(i);
            let left = &left[1..];
            let right = &right[1..right.len()-1];
            return (left, right);
        }
    }
    panic!("Shouldn't happen")
}

#[derive(Eq, PartialEq, Debug, Clone)]
enum Number {
    Literal(i64),
    Pair(Box<Number>, Box<Number>),
}
enum ExplosionResult {
    NoExplosion(Number),
    Exploded(Number),
}

impl Display for ExplosionResult {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            NoExplosion(n) => write!(f, "({})",n),
            ExplosionResult::Exploded(n) => write!(f, "|{}|", n),
        }
    }
}



impl Number {
    pub(crate) fn explode(self, depth: i64) -> ExplosionResult {
        if depth == 4 {
            // time to explode
            return ExplosionResult::Exploded(self);
        }
        // // println!("Exploding {}: {}", depth, self);
        match self {
            Number::Pair(left, right) => {
                match left.explode(depth + 1) {
                    ExplosionResult::Exploded(exploded) => {
                        // // println!("Left pair {} exploded", exploded);
                        return ExplosionResult::NoExplosion(Number::Pair(Box::new(Number::Literal(0)), right));
                    }

                    ExplosionResult::NoExplosion(left) => NoExplosion(Number::Pair(Box::new(left), right)),
                }
            }
            Number::Literal(v) => return ExplosionResult::NoExplosion(self),
        }
    }
}

// }

impl Display for Number {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Number::Literal(v) => write!(f, "{}", v),
            Number::Pair(left, right) => write!(f, "[{},{}]", left, right),
        }
    }
}

impl std::ops::Add for Number {
    type Output = Number;

    fn add(self, rhs: Self) -> Self::Output {
        Number::Pair(Box::new(self), Box::new(rhs))
    }
}

// #[aoc(day18, part2)]
// fn part2(input: &[Line]) -> usize {
//     todo!()
// }

#[cfg(test)]
mod tests {
    use super::*;

#[test]
    fn verify_part1() {
        let input = include_str!("../input/2021/day18.txt");
        assert_eq!(part1(input), 4202);
    }

    #[test]
    fn verify_part2() {
        let input = include_str!("../input/2021/day18.txt");
        assert_eq!(part2(input), 4779);
    }

    #[test]
    fn part1_provided_example() {
        let result = part1(r#"[[[0,[5,8]],[[1,7],[9,6]]],[[4,[1,2]],[[1,4],2]]]
[[[5,[2,8]],4],[5,[[9,9],0]]]
[6,[[[6,2],[5,6]],[[7,6],[4,7]]]]
[[[6,[0,7]],[0,9]],[4,[9,[9,0]]]]
[[[7,[6,4]],[3,[1,3]]],[[[5,5],1],9]]
[[6,[[7,3],[3,2]]],[[[3,8],[5,7]],4]]
[[[[5,4],[7,7]],8],[[8,3],8]]
[[9,3],[[9,9],[6,[4,9]]]]
[[2,[[7,7],7]],[[5,8],[[9,3],[0,2]]]]
[[[[5,2],5],[8,[3,7]]],[[5,[7,5]],[4,4]]]"#);

        assert_eq!(result, 4140)
    }
    //
    // #[test]
    // fn part2_provided_example() {}

    #[test]
    fn also_example() {
        assert_eq!(part1(r#"[[[0,[4,5]],[0,0]],[[[4,5],[2,6]],[9,5]]]
[7,[[[3,7],[4,3]],[[6,3],[8,8]]]]
[[2,[[0,8],[3,4]]],[[[6,7],1],[7,[1,6]]]]
[[[[2,4],7],[6,[0,5]]],[[[6,8],[2,8]],[[2,1],[4,5]]]]
[7,[5,[[3,8],[1,4]]]]
[[2,[2,2]],[8,[8,1]]]
[2,9]
[1,[[[9,3],9],[[9,0],[0,7]]]]
[[[5,[7,4]],7],1]
[[[[4,2],2],6],[8,7]]"#),3488);
    }

    #[test]
    fn just_add() {
        let left = parse("[1,2]");
        let right = parse("[[3,4],5]");
        let expected = parse("[[1,2],[[3,4],5]]");
        // // println!("{} + {} = {}", left, right, expected);
        assert_eq!(left + right, expected);
    }

    #[test]
    fn test_simple_add() {
        assert_eq!(simple_add("[1,2]","[[3,4],5]").as_str(), "[[1,2],[[3,4],5]]");
    }

    #[test]
    fn test_simple_explode() {
        assert_eq!(simple_explode("[[[[[9,8],1],2],3],4]"), "[[[[0,9],2],3],4]");
        assert_eq!(simple_explode("[7,[6,[5,[4,[3,2]]]]]"), "[7,[6,[5,[7,0]]]]");
        assert_eq!(simple_explode("[[6,[5,[4,[3,2]]]],1]"), "[[6,[5,[7,0]]],3]");
        assert_eq!(simple_explode("[[3,[2,[1,[7,3]]]],[6,[5,[4,[3,2]]]]]"), "[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]");
        assert_eq!(simple_explode("[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]"), "[[3,[2,[8,0]]],[9,[5,[7,0]]]]");
    }

    #[test]
    fn test_simple_split() {
        assert_eq!(simple_split("[[[[0,7],4],[[7,8],[0,13]]],[1,1]]"),"[[[[0,7],4],[[7,8],[0,[6,7]]]],[1,1]]");
    }

    #[test]
    fn test_add_and_splitxplode() {
        assert_eq!(proper_add("[[[0,[4,5]],[0,0]],[[[4,5],[2,6]],[9,5]]]", "[7,[[[3,7],[4,3]],[[6,3],[8,8]]]]"), "[[[[4,0],[5,4]],[[7,7],[6,0]]],[[8,[7,7]],[[7,9],[5,0]]]]");
        assert_eq!(proper_add("[[[[4,3],4],4],[7,[[8,4],9]]]", "[1,1]"), "[[[[0,7],4],[[7,8],[6,0]]],[8,1]]");
    }

    #[test]
    fn test_reduce() {
        assert_eq!(reduce("[[[[4,0],[5,4]],[[7,7],[6,0]]],[[8,9],[[[5,6],9],[11,0]]]]"), "[[[[4,0],[5,4]],[[7,7],[6,0]]],[[8,[7,7]],[[7,9],[5,0]]]]");
        assert_eq!(reduce("[[[[4,0],[5,4]],[[7,7],[0,[6,7]]]],[10,[[11,9],[11,0]]]]"), "[[[[4,0],[5,4]],[[7,7],[6,0]]],[[8,[7,7]],[[7,9],[5,0]]]]");
        assert_eq!(reduce("[[[[0,[4,5]],[0,0]],[[[4,5],[2,6]],[9,5]]],[7,[[[3,7],[4,3]],[[6,3],[8,8]]]]]"), "[[[[4,0],[5,4]],[[7,7],[6,0]]],[[8,[7,7]],[[7,9],[5,0]]]]");
    }

    #[test]
    fn test_get_magnitude() {
        assert_eq!(get_magnitude("[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]"), 3488);
    }
    #[test]
    fn test_split_pair() {
        assert_eq!(split_pair("[1,2]"), ("1", "2"));
        assert_eq!(split_pair("[1,[2,3]]"), ("1", "[2,3]"));
        assert_eq!(split_pair("[[1,3],2]"), ("[1,3]", "2"));
        assert_eq!(split_pair("[[[1,2],3],[2,[2,3]]]"), ("[[1,2],3]", "[2,[2,3]]"));
    }

    #[test]
    fn test_explode() {
        let input = parse("[[[[[9,8],1],2],3],4]");
        let expected = parse("[[[[0,9],2],3],4]");

        let result = input.explode(0);
        // // println!("res: {}", result);
    }
}
