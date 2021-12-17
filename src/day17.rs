use std::cmp::Ordering;
use std::str::FromStr;

#[aoc(day17, part1)]
fn part1(input: &str) -> i64 {
    // target area: x=20..30, y=-10..-5
    let input = input.lines().next().unwrap();

    let target_area = Rectangle::from_str(input).unwrap();
    
    println!("area: {:?}", target_area);

    // calculate_max_height(&target_area, (6,9)).unwrap()
    let mut max_height = -100;
    for x in 1..1000 {
        for y in -1000..1000 {
            if let Some(heigh) = calculate_max_height(&target_area, (x,y)) {
                max_height = max_height.max(heigh);
            }
        }
    }
    max_height
}

#[aoc(day17, part2)]
fn part2(input: &str) -> i64 {
    // target area: x=20..30, y=-10..-5
    let input = input.lines().next().unwrap();

    let target_area = Rectangle::from_str(input).unwrap();

    println!("area: {:?}", target_area);

    // calculate_max_height(&target_area, (6,9)).unwrap()
    let mut successful_initial_values = 0;
    for x in 1..1000 {
        for y in -1000..1000 {
            if let Some(_heigh) = calculate_max_height(&target_area, (x, y)) {
                successful_initial_values += 1;
            }
        }
    }
    successful_initial_values
}

fn calculate_max_height(target_area: &Rectangle, initial_velocity: (i64,i64)) -> Option<i64> {
    let (mut dx, mut dy) = initial_velocity;
    let mut path = vec![(0,0)];
    loop {
        let (x,y) = *path.last().unwrap();
        let new_x = x + dx;
        let new_y = y + dy;

        match dx.cmp(&0) {
            Ordering::Less => dx += 1,
            Ordering::Equal => (),
            Ordering::Greater => dx -= 1,
        }

        dy -= 1;

        path.push((new_x, new_y));
        if target_area.inside(new_x,new_y) {
            // println!("{},{} is inside", x,y);
            break
        }
        if x > target_area.x1 || y < target_area.y0 {
            return None;
        }
    }
    // println!("path {}: {:?}", path.len(), path);


    let (_,y_max) = *path.iter().max_by_key(|(_x,y)|y).unwrap();
    let (_,y_min) = *path.iter().min_by_key(|(_x,y)|y).unwrap();

    let (x_max,_) = *path.iter().max_by_key(|(x,_y)|x).unwrap();
    let (x_min,_) = *path.iter().min_by_key(|(x,_y)|x).unwrap();
    let result = y_max;

    let mut y_min = y_min.min(target_area.y0);
    let mut x_max = x_max.max(target_area.x1);

    y_min -= 1;
    x_max += 1;

    // println!("y_min: {}, y_max: {}", y_min, y_max);
    //
    // for y in (y_min..=y_max).rev() {
    //     print!("{:>3} ", y);
    //     for x in x_min..=x_max {
    //         if path.contains(&(x,y)) {
    //             print!("#");
    //         } else if target_area.inside(x,y) {
    //             print!("T");
    //         } else {
    //             print!(".");
    //         }
    //     }
    //     println!()
    // }

    Some(result)
}

#[derive(Debug)]
struct Rectangle {
    x0: i64,
    x1: i64,
    y0: i64,
    y1: i64,
}

impl Rectangle {
    fn inside(&self, x:i64, y:i64) -> bool {
        if x > self.x1 {
            return false;
        }
        if y > self.y1 {
            return false;
        }
        if x < self.x0 {
            return false;
        }
        if y < self.y0 {
            return false;
        }
        true
    }
}

impl FromStr for Rectangle {
    type Err = String;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let input = input.replace("target area: ", "");
        let mut split = input.split(',');
        let x = split.next().unwrap();
        let x = x.replace("x=","");
        let mut x_split = x.split("..");
        let x_start:i64 = x_split.next().unwrap().parse().unwrap();
        let x_end:i64 = x_split.next().unwrap().parse().unwrap();
        let y = split.next().unwrap();
        let y = y.replace(" y=","");
        let mut y_split = y.split("..");
        let y_start:i64 = y_split.next().unwrap().parse().unwrap();
        let y_end:i64 = y_split.next().unwrap().parse().unwrap();

        Ok(Self {
            x0: x_start.min(x_end),
            x1: x_start.max(x_end),
            y0: y_start.min(y_end),
            y1: y_start.max(y_end),
        })
    }
}

// #[aoc(day17, part2)]
// fn part2(input: &[Line]) -> usize {
//     todo!()
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn verify_part1() {
        let input = include_str!("../input/2021/day17.txt");
        let result = part1(input);
        assert_eq!(result, 5995);
    }

    #[test]
    fn verify_part2() {
        let input = include_str!("../input/2021/day17.txt");
        let result = part2(input);
        assert_eq!(result, 3202) // to low
    }

    // #[test]
    // fn verify_part2() {
    //     let input = include_str!("../input/2021/day17.txt");
    //     assert_eq!(part2(parse_input(input).as_slice()), 19081);
    // }

    #[test]
    fn part1_provided_example() {
        let input = r#"target area: x=20..30, y=-10..-5"#;
        let result = part1(input);

        assert_eq!(result, 45);

        let rect = Rectangle::from_str(input).unwrap();

        assert_eq!(calculate_max_height(&rect, (7,2)), Some(3));
        assert_eq!(calculate_max_height(&rect, (6,3)), Some(6));
        assert!(calculate_max_height(&rect, (17,-4)).is_none());

    }

    #[test]
    fn part2_provided_example() {}
}
