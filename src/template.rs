#[aoc_generator(dayX)]
fn parse_input(input: &str) -> Vec<u8> {
    todo!()
}


#[aoc(dayX, part1)]
fn part1(input: &[Line]) -> usize {
    todo!()
}

// #[aoc(dayX, part2)]
// fn part2(input: &[Line]) -> usize {
//     todo!()
// }

#[cfg(test)]
mod tests {
    use super::*;

// #[test]
    // fn verify_part1() {
    //     let input = include_str!("../input/2021/day5.txt");
    //     let input = parse_input(input);
    //     assert_eq!(part1(input.as_slice()), 6666);
    // }

    // #[test]
    // fn verify_part2() {
    //     let input = include_str!("../input/2021/day5.txt");
    //     assert_eq!(part2(parse_input(input).as_slice()), 19081);
    // }

    #[test]
    fn part1_provided_example() {
        let result = part1(&parse_input(
            r#""#,
        ));

        assert_eq!(result, 5)
    }

    #[test]
    fn part2_provided_example() {}
}
