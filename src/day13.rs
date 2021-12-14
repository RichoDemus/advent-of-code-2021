use std::collections::HashSet;

#[aoc_generator(day13)]
fn parse_input(input: &str) -> Input {
    let mut split = input.split("\n\n");
    let coordinates = split.next().unwrap();
    let foldings = split.next().unwrap();

    let coordinates = coordinates
        .lines()
        .map(|line| {
            let mut split = line.split(',');
            let x = split.next().unwrap();
            let y = split.next().unwrap();
            (x.parse().unwrap(), y.parse().unwrap())
        })
        .collect();

    let folds = foldings
        .lines()
        .map(|line| {
            let fold = &line[11..];
            let mut split = fold.split('=');
            let axis = split.next().unwrap();
            let axis = axis.chars().next().unwrap();
            let fold_line = split.next().unwrap();
            let fold_line = fold_line.parse().unwrap();
            (axis, fold_line)
        })
        .collect();
    Input { coordinates, folds }
}

#[derive(Clone, Debug)]
struct Input {
    coordinates: Vec<(i32, i32)>,
    folds: Vec<(char, i32)>,
}

#[aoc(day13, part1)]
fn part1(input: &Input) -> usize {
    let coordinates = input.coordinates.clone();
    // let y_max = *coordinates.iter().map(|(_x,y)|y).max().unwrap();
    // println!("ymax is {}", y_max);
    //
    let (_axis, fold) = *input.folds.get(0).unwrap();

    let result = fold_paper(
        &coordinates.into_iter().collect::<HashSet<_, _>>(),
        fold,
        FoldDirection::Vertical,
    );
    result.len()
}

#[derive(Copy, Clone)]
enum FoldDirection {
    Vertical,
    Horizontal,
}

fn fold_paper(
    points: &HashSet<(i32, i32)>,
    fold_at: i32,
    fold_direction: FoldDirection,
) -> HashSet<(i32, i32)> {
    points
        .iter()
        .map(|(x, y)| match fold_direction {
            FoldDirection::Vertical => {
                if *x > fold_at {
                    (fold_at - (*x - fold_at), *y)
                } else {
                    (*x, *y)
                }
            }
            FoldDirection::Horizontal => {
                if *y > fold_at {
                    (*x, fold_at - (*y - fold_at))
                } else {
                    (*x, *y)
                }
            }
        })
        .collect::<HashSet<(i32, i32)>>()
}

fn print(coordinates: &HashSet<(i32, i32)>) {
    let x_max = *coordinates.iter().map(|(x, _y)| x).max().unwrap();
    let y_max = *coordinates.iter().map(|(_x, y)| y).max().unwrap();

    for y in 0..=y_max {
        for x in 0..=x_max {
            if coordinates.contains(&(x, y)) {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!();
    }
}

#[aoc(day13, part2)]
fn part2(input: &Input) -> usize {
    let coordinates = input.coordinates.clone();
    // let y_max = *coordinates.iter().map(|(_x,y)|y).max().unwrap();
    // println!("ymax is {}", y_max);
    //

    let mut result = coordinates.into_iter().collect::<HashSet<_, _>>();
    for (axis, fold_line) in &input.folds {
        let axis = match axis {
            'y' => FoldDirection::Horizontal,
            'x' => FoldDirection::Vertical,
            _ => panic!(),
        };
        result = fold_paper(&result, *fold_line, axis);
    }
    print(&result);
    result.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn verify_part1() {
        let input = include_str!("../input/2021/day13.txt");
        let input = parse_input(input);
        let result = part1(&input);
        assert_ne!(result, 863);
        assert_eq!(result, 724);
    }

    #[test]
    fn verify_part2() {
        let input = include_str!("../input/2021/day13.txt");
        assert_eq!(part2(&parse_input(input)), 95);
    }

    #[test]
    fn part1_provided_example() {
        let result = part1(&parse_input(
            r#"6,10
0,14
9,10
0,3
10,4
4,11
6,0
6,12
4,1
0,13
10,12
3,4
3,0
8,4
1,10
2,14
8,10
9,0

fold along y=7
fold along x=5"#,
        ));

        assert_eq!(result, 17)
    }

    #[test]
    fn part2_provided_example() {}
}
