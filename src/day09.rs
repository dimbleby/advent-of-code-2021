use std::collections::{HashMap, HashSet};
use std::str::FromStr;

struct HeightMap {
    heights: HashMap<(usize, usize), u32>,
    max_x: usize,
    max_y: usize,
}

impl FromStr for HeightMap {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut heights: HashMap<(usize, usize), u32> = HashMap::new();
        for (y, line) in s.lines().enumerate() {
            for (x, number) in line.chars().enumerate() {
                let height = number.to_digit(10).ok_or(())?;
                heights.insert((x, y), height);
            }
        }

        let height_map = Self::new(heights);
        Ok(height_map)
    }
}

impl HeightMap {
    fn new(heights: HashMap<(usize, usize), u32>) -> Self {
        let &max_x = heights.keys().map(|(x, _)| x).max().unwrap();
        let &max_y = heights.keys().map(|(_, y)| y).max().unwrap();
        Self {
            heights,
            max_x,
            max_y,
        }
    }

    fn neighbours(&self, (x, y): (usize, usize)) -> Vec<(usize, usize)> {
        let mut neighbours = vec![];
        if x > 0 {
            neighbours.push((x - 1, y));
        }
        if x < self.max_x {
            neighbours.push((x + 1, y));
        }
        if y > 0 {
            neighbours.push((x, y - 1));
        }
        if y < self.max_y {
            neighbours.push((x, y + 1));
        }
        neighbours
    }

    fn is_low_point(&self, point: (usize, usize)) -> bool {
        let height = self.heights[&point];
        self.neighbours(point)
            .iter()
            .all(|neighbour| self.heights[neighbour] > height)
    }

    fn basin_size(&self, start_point: (usize, usize)) -> usize {
        let mut basin = HashSet::<(usize, usize)>::new();
        let mut stack = vec![start_point];

        while let Some(point) = stack.pop() {
            if !basin.insert(point) {
                continue;
            }

            for neighbour in self.neighbours(point) {
                if self.heights[&neighbour] != 9 {
                    stack.push(neighbour);
                }
            }
        }

        basin.len()
    }
}

pub(crate) fn day09() {
    let input = std::fs::read_to_string("data/day09.txt").unwrap();
    let height_map: HeightMap = input.parse().unwrap();

    let low_points = itertools::iproduct!((0..=height_map.max_x), (0..=height_map.max_y))
        .filter(|&point| height_map.is_low_point(point));

    let part_one: u32 = low_points
        .clone()
        .map(|point| 1 + height_map.heights[&point])
        .sum();
    println!("Part one answer is {}", part_one);

    let mut basin_sizes: Vec<_> = low_points
        .map(|point| height_map.basin_size(point))
        .collect();
    basin_sizes.sort_unstable();
    basin_sizes.reverse();
    let part_two: usize = basin_sizes.iter().take(3).product();
    println!("Part two answer is {}", part_two);
}
