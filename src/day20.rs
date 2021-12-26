#[aoc_generator(day20)]
fn parse_input(input: &str) -> Input {
    let mut split = input.split("\n\n");
    let image_enhancement_algorithm = split.next().unwrap();
    let input_image = split.next().unwrap();

    let image_enhancement_algorithm = image_enhancement_algorithm.chars()
        .map(|pixel|if pixel == '#' { true} else {false})
        .collect::<Vec<_>>();

    let input_image = input_image.lines()
        .map(|line|line.chars().map(|pixel|if pixel == '#' { true} else {false}).collect::<Vec<_>>())
        .collect::<Vec<_>>();
    Input {
        image_enhancement_algorithm,
        input_image,
    }
}

#[derive(Debug)]
struct Input {
    image_enhancement_algorithm: Vec<bool>,
    input_image: Vec<Vec<bool>>,
}

#[aoc(day20, part1)]
fn part1(input: &Input) -> usize {
    panic!("{:?}", input)
}

fn enhance(image_enhancement_algorithm: &Vec<bool>, input_image: &Vec<Vec<bool>>,) -> Vec<Vec<bool>> {
    todo!()
}

fn add_borders(input_image: Vec<Vec<bool>>) -> Vec<Vec<bool>> {
    let with_horizontal_
    todo!()
}

// #[aoc(day20, part2)]
// fn part2(input: &[Line]) -> usize {
//     todo!()
// }

#[cfg(test)]
mod tests {
    use super::*;

// #[test]
    // fn verify_part1() {
    //     let input = include_str!("../input/2021/day20.txt");
    //     let input = parse_input(input);
    //     assert_eq!(part1(input.as_slice()), 6666);
    // }

    // #[test]
    // fn verify_part2() {
    //     let input = include_str!("../input/2021/day20.txt");
    //     assert_eq!(part2(parse_input(input).as_slice()), 19081);
    // }

    #[test]
    fn part1_provided_example() {
        let result = part1(&parse_input(
            r#"..#.#..#####.#.#.#.###.##.....###.##.#..###.####..#####..#....#..#..##..###..######.###...####..#..#####..##..#.#####...##.#.#..#.##..#.#......#.###.######.###.####...#.##.##..#..#..#####.....#.#....###..#.##......#.....#..#..#..##..#...##.######.####.####.#.#...#.......#..#.#.#...####.##.#......#..#...##.#.##..#...##.#.##..###.#......#.#.......#.#.#.####.###.##...#.....####.#..#..#.##.#....##..#.####....##...##..#...#......#.#.......#.......##..####..#...#.#.#...##..#.#..###..#####........#..####......#..#

#..#.
#....
##..#
..#..
..###"#,
        ));

        assert_eq!(result, 5)
    }

    #[test]
    fn part2_provided_example() {}
}
