use std::collections::HashMap;
use std::fmt::{Display, Formatter};
use std::str::FromStr;

#[derive(Clone)]
pub struct Grid<V: FromStr + std::fmt::Display + std::fmt::Debug> {
    pub grid: HashMap<(i64, i64), V>,
}

impl<V: FromStr + std::fmt::Display + std::fmt::Debug> Grid<V> {
    pub(crate) fn calc_in_bounds_four_way_neighbours(&self, x: i64, y: i64) -> Vec<(i64,i64)> {
        get_four_neighbours(x,y)
            .into_iter()
            .filter(|(x,y)|self.is_within_bounds(*x,*y))
            .collect()

    }
}

impl<V: FromStr + std::fmt::Display + std::fmt::Debug> Grid<V> {
    /// constructs grid from a text block like:
    /// VVVV
    /// VVVV
    /// VVVV
    ///
    pub fn from_non_delim_block(input: &str) -> Self {
        let grid = input
            .lines()
            .enumerate()
            .flat_map(|(y, line)| {
                line.chars()
                    .enumerate()
                    .map(|(x, char)| {
                        let value = V::from_str(char.to_string().as_str());
                        let value = value.unwrap_or_else(|_| panic!("can't parse V"));
                        ((x as i64, y as i64), value)
                    })
                    .collect::<Vec<_>>()
            })
            .collect();
        Self { grid }
    }
    // todo impl
    // pub fn for_each_mut()

    pub(crate) fn x_max(&self) -> i64 {
        *self.grid.keys().map(|(x, _y)| x).max().unwrap()
    }

    pub(crate) fn y_max(&self) -> i64 {
        *self.grid.keys().map(|(_x, y)| y).max().unwrap()
    }

    pub(crate) fn size(&self) -> u64 {
        (self.x_max()+1) as u64 * (self.y_max()+1) as u64
    }

    pub(crate) fn is_within_bounds(&self, x:i64, y:i64) -> bool {
        let x_min = *self.grid.keys().map(|(x, _y)| x).min().unwrap();
        let x_max = *self.grid.keys().map(|(x, _y)| x).max().unwrap();
        let y_min = *self.grid.keys().map(|(_x, y)| y).min().unwrap();
        let y_max = *self.grid.keys().map(|(_x, y)| y).max().unwrap();

        if x < x_min {
            return false;
        }
        if y < y_min {
            return false;
        }
        if x > x_max {
            return false;
        }
        if y > y_max {
            return false;
        }
        true
    }
}

impl<V: std::str::FromStr + std::fmt::Display + std::fmt::Debug> Display for Grid<V> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let x_min = *self.grid.keys().map(|(x, _y)| x).min().unwrap();
        let x_max = *self.grid.keys().map(|(x, _y)| x).max().unwrap();
        let y_min = *self.grid.keys().map(|(_x, y)| y).min().unwrap();
        let y_max = *self.grid.keys().map(|(_x, y)| y).max().unwrap();

        let mut output = String::new();
        for y in y_min..=y_max {
            for x in x_min..=x_max {
                let value = self
                    .grid
                    .get(&(x, y))
                    .unwrap_or_else(|| panic!("No value at {},{}: {:?}", x, y, self.grid));
                output += format!("{}", value).as_str();
            }
            output += "\n";
        }
        write!(f, "{}", output)
    }
}

pub fn get_eight_neighbours(x: i64, y: i64) -> Vec<(i64, i64)> {
    let mut result = vec![];
    for dx in -1..=1 {
        for dy in -1..=1 {
            if dx == 0 && dy == 0 {
                continue;
            }
            result.push((x + dx, y + dy));
        }
    }
    result
}

pub fn get_four_neighbours(x: i64, y: i64) -> Vec<(i64, i64)> {
    vec![
        (x+1, y),
        (x, y+1),
        (x-1, y),
        (x, y-1),
    ]
}
